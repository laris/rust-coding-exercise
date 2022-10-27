
struct GpuModule {
    pn: Option<String>,
    dsc: Option<String>,
    sn: Option<String>,
    vbios: Option<String>,
}

enum GpuTrayLocation {
    Upper(2),
    Lower(1),
}

struct GpuSlot {
    gpum: GpuModule,
    log_id: Option<u8>,
    phy_id: u8,
    pcie_bdf: Option<String>,
    tray_loc: GpuTrayLocation,
}

type GpuId  = u8;
type NvLinkId = u8;
type NvSwitchId = u8;
type PcieBdf = u8;

enum NvLinkConDev {
    Gpu{ bdf: PcieBdf, gpuid: GpuId, nvlnkid: NvLinkId, },
    Nvs{ bdf: PcieBdf, nvsid: NvSwitchId, nvlnkid: NvLinkId },
    Unused, 
}

struct NvLink {
    id: u8, // [0, 17]
    p1: NvLinkConDev,
    p2: NvlinkConDev,
}

struct Nvs {
    id: NvSwitchId,
    nvl: NvLink,
}

struct Gpu {
    id: GpuId,
    nvl: NvLink,
}

