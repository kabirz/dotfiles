pub fn initialize_system_clocks(rcc: Rcc, vos: VoltageScale, syscfg: &SYSCFG) -> Ccdr {
    let ccdr = rcc
        .use_hse(16.mhz()) // XTAL X1
        .sys_ck(400.mhz())
        .pll1_r_ck(400.mhz()) // for TRACECK
        .freeze(vos, syscfg);

    ccdr
}

/// Enables ITM
unsafe fn enable_itm(
    dbgmcu: &stm32h7xx_hal::stm32::DBGMCU,
    dcb: &mut cortex_m::peripheral::DCB,
    itm: &mut cortex_m::peripheral::ITM,
    swo_frequency: u32,
) {
    // ARMv7-M DEMCR: Set TRCENA. Enables DWT and ITM units
    //unsafe { *(0xE000_EDFC as *mut u32) |= 1 << 24 };
    dcb.enable_trace();

    // Ensure debug blocks are clocked before interacting with them
    dbgmcu.cr.modify(|_, w| {
        w.d1dbgcken()
            .set_bit()
            .d3dbgcken()
            .set_bit()
            .traceclken()
            .set_bit()
            .dbgsleep_d1()
            .set_bit()
    });

    // SWO: Unlock
    *(0x5c00_3fb0 as *mut u32) = 0xC5ACCE55;
    // SWTF: Unlock
    *(0x5c00_4fb0 as *mut u32) = 0xC5ACCE55;

    // SWO CODR Register: Set SWO speed
    *(0x5c00_3010 as *mut _) = crate::system::clock::get_clocks().c_ck().0 / swo_frequency - 1;

    // SWO SPPR Register:
    // 1 = Manchester
    // 2 = NRZ
    *(0x5c00_30f0 as *mut _) = 2;

    // SWTF Trace Funnel: Enable for CM7
    *(0x5c00_4000 as *mut u32) |= 1;

    // ITM: Unlock
    itm.lar.write(0xC5ACCE55);
    // ITM Trace Enable Register: Enable lower 8 stimulus ports
    itm.ter[0].write(1);
    // ITM Trace Control Register: Enable ITM
    itm.tcr.write(
        (0b000001 << 16) | // TraceBusID
        (1 << 3) | // enable SWO output
        (1 << 0), // enable the ITM
    );
}
