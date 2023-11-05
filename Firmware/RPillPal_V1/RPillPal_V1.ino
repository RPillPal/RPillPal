#include <Wire.h>
#include <OneButton.h>
#include <Adafruit_GFX.h>
#include <Adafruit_SSD1306.h>
#include <Stepper.h>

#define SCREEN_WIDTH 128
#define SCREEN_HEIGHT 32

Adafruit_SSD1306 display(SCREEN_WIDTH, SCREEN_HEIGHT, &Wire, -1);

 
#define KEY1 13
#define KEY2 12
#define KEY3 11
#define KEYPOUND 9

#define BUZZER 10


const uint16_t stepsPerRevolution = 2038;
Stepper myStepper = Stepper(stepsPerRevolution, 4, 6, 5, 7);

OneButton K1 = OneButton(
  KEY1,
  true,
  true
);
OneButton K2 = OneButton(
  KEY2,
  true,
  true
);
OneButton K3 = OneButton(
  KEY3,
  true,
  true
);
OneButton KP = OneButton(
  KEYPOUND,
  true,
  true
);

uint8_t CORR_ID[3] = {1, 2, 3};
uint8_t ID[3];
uint8_t count = 0;

uint16_t pillCount = 120;

void dispensePill(uint8_t amount) {
  myStepper.setSpeed(5);
  for (int i = 0; i < amount; i++) {
    Serial.println("Yeet");
	  myStepper.step(stepsPerRevolution);
    delay(1000);
  }
}

static void numKeyPress(uint8_t num) {
  ID[count] = num;
  count += 1;
  tone(BUZZER, 1000, 200);
  if (count == 3) {
    keyPoundPress();
  }
}

static void key1Press() {numKeyPress(1);}
static void key2Press() {numKeyPress(2);}
static void key3Press() {numKeyPress(3);}

static void keyPoundPress() {
  count = 0;
  bool wrongFlag = false;
  for (int i = 0; i < 3; i++) {
    if (ID[i] != CORR_ID[i]) {
      wrongFlag = true;
    }
    Serial.print(ID[i]);
  }
  if (wrongFlag) {
    tone(BUZZER, 900, 200);
    Serial.println("Incorrect Password");
  }
  else {
    tone(BUZZER, 1500, 200);
    Serial.println("Correct Password");
    dispensePill(3);
  }
  Serial.println("");
}

void displayData() {
  display.clearDisplay();

  display.setTextSize(1);
  int batteryBars = random(0, 5);

  display.drawRoundRect(34, 0, 94, 32, 4, 1);
  int startX = 37;
  for (int i = 0; i < batteryBars; i++) {
    display.fillRoundRect(startX, 2, 7, 28, 2, 1);
    startX += 9;
  }
  
  display.setTextColor(WHITE);
  display.setCursor(40, 0);
  display.print(F("TRB: ")); 
  display.setCursor(85, 0);
  display.setCursor(40, 18); 
  display.print(F("OUT: "));     
  display.setCursor(85, 18);
  display.print("5V"); 
  
  display.display();
}

void setup() {
  // put your setup code here, to run once:
  Serial.begin(9600);
  if(!display.begin(SSD1306_SWITCHCAPVCC, 0x3C)) {
    Serial.println(F("SSD1306 allocation failed"));
    for(;;);
  }
  delay(2000); // Pause for 2 seconds
 
  // Clear the buffer.
  display.clearDisplay();
  display.setFont();
  display.setTextSize(1);

  // // Draw bitmap on the screen
  // display.drawBitmap(0, 5, epd_bitmap_ANAVEO_128x32, 128, 32, WHITE);
  // display.display();
  // // delay(4000);
  
  K1.attachClick(key1Press);
  K2.attachClick(key2Press);
  K3.attachClick(key3Press);
  KP.attachClick(keyPoundPress);

  pinMode(BUZZER, OUTPUT);
  displayData();
}

void loop() {
  K1.tick();
  K2.tick();
  K3.tick();
  KP.tick();
}
