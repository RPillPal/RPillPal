#ifndef INIT_H_
#define INIT_H_

#include "stm32f769xx.h"
#include "stm32f7xx_hal.h"
#include "uart.h"

//Function Prototypes
void SystemClock_Config(void);
void CPU_CACHE_Enable(void);
void Sys_Init(void);
void Clock_Inits(void);

#endif
