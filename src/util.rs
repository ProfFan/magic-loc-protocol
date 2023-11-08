use dw3000::Config;

/// Calculate frame TX time in nanoseconds
pub fn frame_tx_time(mut frame_len: u32, config: &Config, include_body: bool) -> u32 {
    let mut tx_time = 0u32;
    let mut shr_len = 0u32;
    let mut sym_timing_ind = 0;

    const DATA_BLOCK_SIZE: u32 = 330;
    const REED_SOLOM_BITS: u32 = 48;

    // Symbol timing LUT
    const SYM_TIM_16MHZ: u32 = 0;
    const SYM_TIM_64MHZ: u32 = 9;
    const _SYM_TIM_110K: u32 = 0; // unsupported by DW3000
    const SYM_TIM_850K: u32 = 3;
    const SYM_TIM_6M8: u32 = 6;
    const SYM_TIM_SHR: u32 = 0;
    const SYM_TIM_PHR: u32 = 1;
    const SYM_TIM_DAT: u32 = 2;

    const SYM_TIM_LUT: [u32; 18] = [
        // 16 MHz PRF
        994, 8206, 8206, // 0.11 Mbps
        994, 1026, 1026, // 0.85 Mbps
        994, 1026, 129, // 6.81 Mbps
        // 64 MHz PRF
        1018, 8206, 8206, // 0.11 Mbps
        1018, 1026, 1026, // 0.85 Mbps
        1018, 1026, 129, // 6.81 Mbps
    ];

    match config.pulse_repetition_frequency {
        dw3000::configs::PulseRepetitionFrequency::Mhz16 => sym_timing_ind = SYM_TIM_16MHZ,
        dw3000::configs::PulseRepetitionFrequency::Mhz64 => sym_timing_ind = SYM_TIM_64MHZ,
    }

    // set shr_len
    match config.preamble_length {
        dw3000::configs::PreambleLength::Symbols32 => shr_len = 32,
        dw3000::configs::PreambleLength::Symbols64 => shr_len = 64,
        dw3000::configs::PreambleLength::Symbols72 => shr_len = 72,
        dw3000::configs::PreambleLength::Symbols128 => shr_len = 128,
        dw3000::configs::PreambleLength::Symbols256 => shr_len = 256,
        dw3000::configs::PreambleLength::Symbols512 => shr_len = 512,
        dw3000::configs::PreambleLength::Symbols1024 => shr_len = 1024,
        dw3000::configs::PreambleLength::Symbols1536 => shr_len = 1536,
        dw3000::configs::PreambleLength::Symbols2048 => shr_len = 2048,
        dw3000::configs::PreambleLength::Symbols4096 => shr_len = 4096,
    }

    match config.bitrate {
        dw3000::configs::BitRate::Kbps850 => {
            sym_timing_ind += SYM_TIM_850K;
            shr_len += 8
        }
        dw3000::configs::BitRate::Kbps6800 => {
            sym_timing_ind += SYM_TIM_6M8;
            shr_len += 8
        }
    }

    tx_time = shr_len * SYM_TIM_LUT[(sym_timing_ind + SYM_TIM_SHR) as usize];

    if include_body {
        // Add the PHR time (21 bits)
        tx_time += 21 * SYM_TIM_LUT[(sym_timing_ind + SYM_TIM_PHR) as usize];

        // Bytes to bits
        frame_len *= 8;

        // Add Reed-Solomon parity bits
        frame_len += REED_SOLOM_BITS * (frame_len + DATA_BLOCK_SIZE - 1) / DATA_BLOCK_SIZE;

        // Add the DAT time
        tx_time += frame_len * SYM_TIM_LUT[(sym_timing_ind + SYM_TIM_DAT) as usize];
    }

    return tx_time;
}
