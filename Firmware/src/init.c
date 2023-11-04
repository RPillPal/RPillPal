#include "init.h"

/**
  * @brief  System Clock Configuration
  *         The system Clock is configured as follow :
  *            System Clock source            = PLL (HSE)
  *            SYSCLK(Hz)                     = 216000000
  *            HCLK(Hz)                       = 216000000
  *            AHB Prescaler                  = 1
  *            APB1 Prescaler                 = 4
  *            APB2 Prescaler                 = 2
  *            HSE Frequency(Hz)              = 25000000
  *            PLL_M                          = 25
  *            PLL_N                          = 432
  *            PLL_P                          = 2
  *            PLL_Q                          = 9
  *            PLL_R                          = 7
  *            VDD(V)                         = 3.3
  *            Main regulator output voltage  = Scale1 mode
  *            Flash Latency(WS)              = 7
  * @param  None
  * @retval None
  */
void SystemClock_Config(void) {
  RCC_ClkInitTypeDef RCC_ClkInitStruct;
  RCC_OscInitTypeDef RCC_OscInitStruct;
  HAL_StatusTypeDef ret = HAL_OK;

  /* Enable HSE Oscillator and activate PLL with HSE as source */
  RCC_OscInitStruct.OscillatorType = RCC_OSCILLATORTYPE_HSE;
  RCC_OscInitStruct.HSEState = RCC_HSE_ON;
  RCC_OscInitStruct.PLL.PLLState = RCC_PLL_ON;
  RCC_OscInitStruct.PLL.PLLSource = RCC_PLLSOURCE_HSE;
  RCC_OscInitStruct.PLL.PLLM = 25;
  RCC_OscInitStruct.PLL.PLLN = 432;
  RCC_OscInitStruct.PLL.PLLP = RCC_PLLP_DIV2;
  RCC_OscInitStruct.PLL.PLLQ = 9;
  RCC_OscInitStruct.PLL.PLLR = 7;

  ret = HAL_RCC_OscConfig(&RCC_OscInitStruct);
  if(ret != HAL_OK) {
    while(1) { ; }
  }

  /* Activate the OverDrive to reach the 216 MHz Frequency */
  ret = HAL_PWREx_EnableOverDrive();
  if(ret != HAL_OK) {
    while(1) { ; }
  }

  /* Select PLL as system clock source and configure the HCLK, PCLK1 and PCLK2 clocks dividers */
  RCC_ClkInitStruct.ClockType = (RCC_CLOCKTYPE_SYSCLK | RCC_CLOCKTYPE_HCLK | RCC_CLOCKTYPE_PCLK1 | RCC_CLOCKTYPE_PCLK2);
  RCC_ClkInitStruct.SYSCLKSource = RCC_SYSCLKSOURCE_PLLCLK;
  RCC_ClkInitStruct.AHBCLKDivider = RCC_SYSCLK_DIV1;
  RCC_ClkInitStruct.APB1CLKDivider = RCC_HCLK_DIV4;
  RCC_ClkInitStruct.APB2CLKDivider = RCC_HCLK_DIV2;

  ret = HAL_RCC_ClockConfig(&RCC_ClkInitStruct, FLASH_LATENCY_7);
  if(ret != HAL_OK) {
    while(1) { ; }
  }
}

// Enables CPU Instruction and Data Caches
void CPU_CACHE_Enable(void) {
  /* Enable I-Cache */
  SCB_EnableICache();

  /* Enable D-Cache */
  SCB_EnableDCache();
}

// Unified System Initialization (equivalent of current MPS Sys_Init())
void Sys_Init(void) {
	//Initialize the system
	CPU_CACHE_Enable();		// Enable CPU Caching
	HAL_Init();				// Initialize HAL
	SystemClock_Config(); 	// Configure the system clock to 216 MHz
	Clock_Inits();

	/* UART configured as follows:
		- Word Length = 8 Bits
		- Stop Bit = No Stop bits
		- Parity = None
		- BaudRate = 115200 baud
		- Hardware flow control disabled (RTS and CTS signals)
	*/
	initUart(&USB_UART, 115200, USART1);
	setbuf(stdout, NULL);
}

// Initializes clocks for various peripherals. Some might need to be added!
void Clock_Inits(void) {
	// Clock all GPIO ports and Timers.
	__HAL_RCC_GPIOA_CLK_ENABLE();
	__HAL_RCC_GPIOB_CLK_ENABLE();
	__HAL_RCC_GPIOC_CLK_ENABLE();
	__HAL_RCC_GPIOD_CLK_ENABLE();
	__HAL_RCC_GPIOE_CLK_ENABLE();
	__HAL_RCC_GPIOF_CLK_ENABLE();
	__HAL_RCC_GPIOG_CLK_ENABLE();
	__HAL_RCC_GPIOH_CLK_ENABLE();
	__HAL_RCC_GPIOI_CLK_ENABLE();
	__HAL_RCC_GPIOJ_CLK_ENABLE();
	__HAL_RCC_GPIOK_CLK_ENABLE();
	__HAL_RCC_SYSCFG_CLK_ENABLE();
	__HAL_RCC_TIM1_CLK_ENABLE();
	__HAL_RCC_TIM2_CLK_ENABLE();
	__HAL_RCC_TIM3_CLK_ENABLE();
	__HAL_RCC_TIM4_CLK_ENABLE();
	__HAL_RCC_TIM5_CLK_ENABLE();
	__HAL_RCC_TIM6_CLK_ENABLE();
	__HAL_RCC_TIM7_CLK_ENABLE();
	__HAL_RCC_TIM8_CLK_ENABLE();
	__HAL_RCC_TIM9_CLK_ENABLE();
	__HAL_RCC_TIM10_CLK_ENABLE();
	__HAL_RCC_TIM11_CLK_ENABLE();
	__HAL_RCC_TIM12_CLK_ENABLE();
	__HAL_RCC_TIM13_CLK_ENABLE();
	__HAL_RCC_TIM14_CLK_ENABLE();

	// Enable SPI2 clock
	__HAL_RCC_SPI2_CLK_ENABLE();



}

// This function is what makes everything work
// Don't touch it...
// (Increments the system clock)
void SysTick_Handler(void) {
  HAL_IncTick();
}
