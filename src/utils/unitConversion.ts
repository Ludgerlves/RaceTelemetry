// src/utils/unitConversion.ts
// Conversões de unidades para suporte métrico/imperial.
// Valores brutos no store permanecem em unidades métricas (como vêm do Rust).
// A conversão é aplicada apenas na camada de exibição.

export type UnitSystem = 'metric' | 'imperial';

/** km/h → mph */
export const kmhToMph = (kmh: number): number => kmh * 0.621371;

/** mph → km/h */
export const mphToKmh = (mph: number): number => mph / 0.621371;

/** °C → °F */
export const celsiusToFahrenheit = (c: number): number => (c * 9) / 5 + 32;

/** °F → °C */
export const fahrenheitToCelsius = (f: number): number => ((f - 32) * 5) / 9;

/** kg → lbs */
export const kgToLbs = (kg: number): number => kg * 2.20462;

/** lbs → kg */
export const lbsToKg = (lbs: number): number => lbs / 2.20462;

/** PSI → bar (imperial keeps PSI per settings UI label) */
export const psiToBar = (psi: number): number => psi / 14.5038;

/** bar → PSI */
export const barToPsi = (bar: number): number => bar * 14.5038;

/**
 * Converte um valor de velocidade conforme o sistema de unidades.
 * Retorna o valor convertido e o rótulo da unidade.
 */
export const convertSpeed = (kmh: number, units: UnitSystem) => {
    if (units === 'imperial') {
        return { value: kmhToMph(kmh), unit: 'mph' as const };
    }
    return { value: kmh, unit: 'km/h' as const };
};

/**
 * Converte um valor de temperatura conforme o sistema de unidades.
 */
export const convertTemperature = (celsius: number, units: UnitSystem) => {
    if (units === 'imperial') {
        return { value: celsiusToFahrenheit(celsius), unit: '°F' as const };
    }
    return { value: celsius, unit: '°C' as const };
};

/**
 * Converte um valor de combustível (kg) conforme o sistema de unidades.
 */
export const convertFuel = (kg: number, units: UnitSystem) => {
    if (units === 'imperial') {
        return { value: kgToLbs(kg), unit: 'lbs' as const };
    }
    return { value: kg, unit: 'kg' as const };
};

/**
 * Formata um valor de pressão conforme o sistema de unidades.
 * imperial → PSI (valor bruto)
 * metric   → bar (convertido de PSI)
 */
export const formatPressure = (psi: number, units: UnitSystem) => {
    if (units === 'imperial') {
        return { value: psi, unit: 'PSI' as const };
    }
    return { value: psiToBar(psi), unit: 'bar' as const };
};
