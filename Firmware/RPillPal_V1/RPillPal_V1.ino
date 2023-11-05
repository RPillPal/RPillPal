#include <ArduinoUniqueID.h>

#include <ArduinoJson.h>
#include <ArduinoJson.hpp>
#include <ArduinoHttpClient.h>
#include <WiFiNINA.h>
#include <WiFiUdp.h>
#include <Wire.h>
#include <OneButton.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>
#include <Stepper.h>

#include <time.h>

// Pin Definitions
#define KEY1 13
#define KEY2 12
#define KEY3 11
#define KEYPOUND 9
#define BUZZER 10

// OLED Display Definitions
#define SCREEN_WIDTH 128
#define SCREEN_HEIGHT 32
Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, -1);

// Stepper Definitions
#define STEPREV 2038
Stepper myStepper = Stepper(STEPREV, 4, 6, 5, 7);

// WiFi SSID and Password
char ssid[] = "Glo Dev";
char pass[] = "omsohamom";

// HTTP Server Details
char server[] = "34.86.88.18";
// char server[] = "rpillpal.us";
int port = 5000;

// WiFi Client Definitions
WiFiClient wifi;
HttpClient client = HttpClient(wifi, server, port);

WiFiUDP Udp;
unsigned int localPort = 2390;      // local port to listen for UDP packets
IPAddress timeServer(129, 6, 15, 28); // time.nist.gov NTP server
const int NTP_PACKET_SIZE = 48; // NTP time stamp is in the first 48 bytes of the message
byte packetBuffer[ NTP_PACKET_SIZE]; //buffer to hold incoming and outgoing packets

// Button Definitions
OneButton K1 = OneButton(KEY1, true, true);
OneButton K2 = OneButton(KEY2, true, true);
OneButton K3 = OneButton(KEY3, true, true);
OneButton KP = OneButton(KEYPOUND, true, true);

#define PASSLEN 5
uint8_t CORR_ID[PASSLEN] = {1, 2, 3, 1, 2}; // The correct password
uint8_t ID[PASSLEN]; // The password array
uint8_t count = 0;

struct pillData {
  String name = "Walter"; // Patient name
  int pin[5] = {1,2,3,1,2}; // Password
  String prescriptionName = "Vicodin"; // Pill name
  int numPills = "69"; // Number of pills
  time_t lastTaken; // Last time dose taken (UNIX)
  String dosage = "100mg"; // Patient dosage
  int frequency = 1; // Patient frequency of dosage
  bool canTakePill = false; // Approval to take pill
};

pillData pd;

unsigned long startMillis, currMillis;
const unsigned long ntpInterval = 10000;

unsigned long epoch;
unsigned long epochMillis;
const unsigned long epochInterval = 1000;
int heartbeatCount = 0;
int heartbeatMax = 30;

String names[] = {"Walter", "Jimmy", "Gustavo" , "Howard"};
int nameIdx = 0;
String uniqueIDString = ""; // UID

// Spin the motor based on number of pills needed
void dispensePill(uint8_t amount) {
  myStepper.setSpeed(5);
  for (int i = 0; i < amount; i++) {
	  myStepper.step(STEPREV);
    delay(1000);
  }
}

// Button pressed callback
static void numKeyPress(uint8_t num) {
  ID[count] = num;
  count += 1;
  tone(BUZZER, 1000, 200);
  // Once password length reached, proceed with auth
  if (count == PASSLEN) {
    keyPoundPress();
  }
}

// Used to differentiate between buttons
static void key1Press() {numKeyPress(1);}
static void key2Press() {numKeyPress(2);}
static void key3Press() {numKeyPress(3);}

// Callback when password is typed
static void keyPoundPress() {
  count = 0; // Reset index counter
  nameIdx += 1; // Increment name counter

  // Check for correct password
  bool wrongFlag = false;
  for (int i = 0; i < PASSLEN; i++) {
    if (ID[i] != CORR_ID[i]) {
      wrongFlag = true;
    }
    Serial.print(ID[i]);
  }
  if (wrongFlag) {
    tone(BUZZER, 900, 200);
    Serial.println("Incorrect Password");
    return;
  }

  tone(BUZZER, 1500, 200);
  Serial.println("Correct Password");
  if (!getData()) {
    Serial.print("Get Failed");
    return; 
  }
  if (!pd.canTakePill) { 
    tone(BUZZER, 700, 200);
    Serial.print("Not Approved");
    return; 
  } // Check for approval from database

  // Subtract pills used
  uint8_t dispPills = 1;
  pd.numPills -= dispPills;

  if (!postData()) { 
    Serial.print("Post Failed");
    pd.numPills += dispPills; // Restore old value
    return; 
  } // Check for successful acknowledge sent

  // Decrement pills, update display and dispense
  tone(BUZZER, 2000, 200);
  pd.canTakePill = false;
  pd.lastTaken = epoch;
  displayData();
  dispensePill(dispPills);
}

// Send data to server
bool postData() {
  Serial.println("Making POST request...");

  // Serialize the new values
  String output;
  StaticJsonDocument<128> doc;
  doc["name"] = pd.name;
  doc["numPills"] = pd.numPills;
  doc["timeDispensed"] = epoch;
  serializeJson(doc, output);
  Serial.println(output);

  // Send data to server
  client.beginRequest();
  client.post("/update");
  client.sendHeader(HTTP_HEADER_CONTENT_TYPE, "application/json");
  client.sendHeader(HTTP_HEADER_CONTENT_LENGTH, output.length());
  client.sendHeader("Connection", "close");
  client.write((const byte*)output.c_str(), output.length());

  // Get status code
  int statusCode = client.responseStatusCode();
  Serial.print("HTTP Status Code: ");
  Serial.println(statusCode);
  client = HttpClient(wifi, server, port);

  // Error handling
  if (statusCode != 200) { return false; }

  return true;
}

// Send heartbeat to server
bool postHeartbeat() {
  Serial.println("Making POST request to Heartbeat...");

  // Serialize the new values
  String output;
  StaticJsonDocument<128> doc;
  doc["deviceId"] = uniqueIDString;
  doc["lastHeartbeat"] = epoch;
  serializeJson(doc, output);
  Serial.println(output);

  // Send data to server
  client.beginRequest();
  client.post("/get_devices");
  client.sendHeader(HTTP_HEADER_CONTENT_TYPE, "application/json");
  client.sendHeader(HTTP_HEADER_CONTENT_LENGTH, output.length());
  client.sendHeader("Connection", "close");
  client.write((const byte*)output.c_str(), output.length());

  // Get status code
  int statusCode = client.responseStatusCode();
  Serial.print("HTTP Status Code: ");
  Serial.println(statusCode);
  client = HttpClient(wifi, server, port);

  // Error handling
  if (statusCode != 200) { return false; }

  return true;
}

// Get data from server
bool getData() {
  Serial.println("Making GET request...");
  client.get("/pill_data/" + names[nameIdx]);

  // Get status code
  int statusCode = client.responseStatusCode();
  Serial.print("HTTP Status Code: ");
  Serial.println(statusCode);

  // Error handling
  if (statusCode != 200) { return false; }

  // Get response
  String response = client.responseBody();
  Serial.println("Response: ");
  Serial.println(response);

  // Initialize JSON and deserialize
  StaticJsonDocument<256> doc;
  DeserializationError error = deserializeJson(doc, response);
  
  if (error) {
    Serial.print(F("Getting Pill Data failed: "));
    Serial.println(error.f_str());
    return false;
  }

  // Copy data from JSON to pillData struct
  const char* nm = doc["name"];
  pd.name = String(nm);
  int pinRaw = doc["pin"];
  for (int i = 5 - 1; i >= 0; i--) {
    pd.pin[i] = pinRaw % 10;
    pinRaw /= 10;
  }
  const char* pnm = doc["prescriptionName"];
  pd.prescriptionName = String(pnm);
  pd.numPills = doc["numPills"];
  pd.lastTaken = doc["lastTaken"];
  const char* ds = doc["dosage"];
  pd.dosage = String(ds);
  pd.frequency = doc["frequency"];
  pd.canTakePill = doc["canTakePill"];
  doc.clear();
  client.endRequest();
  return true;
}

void displayData() {
  display.clearDisplay();
  display.setCursor(5, 5);
  display.print(pd.name);
  display.setCursor(5, 20);
  display.print(pd.prescriptionName);
  display.setCursor(60, 20);
  display.print("PILLS:");
  display.setCursor(99, 20);
  display.print(pd.numPills);
  display.display();
}

unsigned long parseNTPpacket() {
    // We've received a packet, read the data from it
    Udp.read(packetBuffer, NTP_PACKET_SIZE); // read the packet into the buffer
    unsigned long highWord = word(packetBuffer[40], packetBuffer[41]);
    unsigned long lowWord = word(packetBuffer[42], packetBuffer[43]);
    unsigned long secsSince1900 = highWord << 16 | lowWord;

    // Unix time starts on Jan 1 1970. In seconds, that's 2208988800:
    const unsigned long seventyYears = 2208988800UL;
    unsigned long epoch = secsSince1900 - seventyYears;
    return epoch;
}

// Send an NTP request to the time server at the given address
unsigned long sendNTPpacket(IPAddress& address) {
  memset(packetBuffer, 0, NTP_PACKET_SIZE);

  // Initialize values needed to form NTP request
  packetBuffer[0] = 0b11100011;   // LI, Version, Mode
  packetBuffer[1] = 0;     // Stratum, or type of clock
  packetBuffer[2] = 6;     // Polling Interval
  packetBuffer[3] = 0xEC;  // Peer Clock Precision
  packetBuffer[12]  = 49;
  packetBuffer[13]  = 0x4E;
  packetBuffer[14]  = 49;
  packetBuffer[15]  = 52;

  // Send packet requesting timestamp
  Udp.beginPacket(address, 123); //NTP requests are to port 123
  Udp.write(packetBuffer, NTP_PACKET_SIZE);
  Udp.endPacket();
}

void setup() {
  Serial.begin(9600);

  // Collect UID
	UniqueID8dump(Serial);
	Serial.print("UniqueID: ");
	for (size_t i = 0; i < 8; i++)
	{
		if (UniqueID8[i] < 0x10)
			Serial.print("0");
		Serial.print(UniqueID8[i], HEX);
		Serial.print(" ");
	}
	Serial.println();

  // Append each byte to the String
  for (int i = 0; i < 16; i++) {
    // Use the HEX format to convert each byte to a hexadecimal string
    uniqueIDString += String(UniqueID8[i], HEX);
  }

  if(!display.begin(SSD1306_SWITCHCAPVCC, 0x3C)) {
    Serial.println(F("SSD1306 allocation failed"));
    for(;;);
  }
  delay(1000); // Pause for 2 seconds
 
  // Clear the buffer.
  display.clearDisplay();
  display.setFont();
  display.setTextSize(1);
  display.setTextColor(WHITE);
  
  // Connect to Wi-Fi
  while (WiFi.begin(ssid, pass) != WL_CONNECTED) {
    Serial.println("Connecting to WiFi...");
    delay(1000);
  }
  Serial.println("Connected to WiFi");
  Udp.begin(localPort);

  // Get current time from NTP server
  sendNTPpacket(timeServer);
  delay(1000);
  if (Udp.parsePacket()) {
    epoch = parseNTPpacket();
  }
  Serial.print("Current Time: ");
  Serial.println(epoch);

  getData(); // Get initial patient data from server

  // Attach callback functions for buttons
  K1.attachClick(key1Press);
  K2.attachClick(key2Press);
  K3.attachClick(key3Press);
  KP.attachClick(keyPoundPress);

  pinMode(BUZZER, OUTPUT);
  displayData();
}

void loop() {
  currMillis = millis();

  // Update time based on timer (dead-reckoning)
  if (currMillis - epochMillis >= epochInterval) {
    epochMillis = currMillis;
    epoch += 1;
    heartbeatCount += 1;

    // Send heartbeat message to server (to indicate device state)
    if (heartbeatCount == heartbeatMax) {
      heartbeatCount = 0;
      postHeartbeat();
    }
  }

  // Update time based on NTP
  if (currMillis - startMillis >= ntpInterval) {
    startMillis = currMillis;
    sendNTPpacket(timeServer); // send an NTP packet to a time server
  }
  if (Udp.parsePacket()) {
    epoch = parseNTPpacket();
  }

  // Update button timers
  K1.tick();
  K2.tick();
  K3.tick();
  KP.tick();
}
