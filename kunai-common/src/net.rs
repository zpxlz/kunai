use kunai_macros::{BpfError, StrEnum};

use crate::{errors::ProbeError, macros::bpf_target_code, macros::not_bpf_target_code};

not_bpf_target_code! {
    mod user;
}

bpf_target_code! {
    mod bpf;
}

#[repr(C)]
#[derive(BpfError, Debug, Clone, Copy)]
pub enum Error {
    #[error("sa_family not supported")]
    UnsupportedSaFamily,
    #[error("sk_type member not found")]
    SkTypeMissing,
    #[error("sk_protocol member not found")]
    SkProtocolMissing,
    #[error("skc_family member not found")]
    SkcFamilyMissing,
    #[error("skc_addrpair member not found")]
    SkcAddrPairMissing,
    #[error("skc_portpair member not found")]
    SkcPortPairMissing,
    #[error("skc_v6_daddr member not found")]
    SkcV6daddrMissing,
    #[error("sockaddr sa_family member not found")]
    SaFamilyMissing,
    #[error("sockaddr_in addr member not found")]
    SaInAddrMissing,
    #[error("sockaddr_in port member not found")]
    SaInPortMissing,
    #[error("sockaddr_in6 addr member not found")]
    SaIn6AddrMissing,
    #[error("sockaddr_in6 port member not found")]
    SaIn6PortMissing,
}

impl From<Error> for ProbeError {
    fn from(value: Error) -> Self {
        Self::IpError(value)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IpType {
    V4,
    V6,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SockAddr {
    pub ty: IpType,
    data: [u32; 4],
    port: u16,
}

impl Default for SockAddr {
    fn default() -> Self {
        Self {
            ty: IpType::V4,
            data: [0; 4],
            port: 0,
        }
    }
}

impl SockAddr {
    pub fn new_v4_from_be(addr: u32, port: u16) -> Self {
        SockAddr {
            ty: IpType::V4,
            data: [addr, 0, 0, 0],
            port,
        }
    }

    pub fn new_v6_from_be(addr: [u32; 4], port: u16) -> Self {
        SockAddr {
            ty: IpType::V6,
            data: addr,
            port,
        }
    }

    pub fn ip(&self) -> u128 {
        match self.ty {
            IpType::V4 => self.data[0] as u128,
            IpType::V6 => u128::from_be_bytes(unsafe {
                core::mem::transmute::<[u32; 4], [u8; 16]>(self.data)
            }),
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn is_v4(&self) -> bool {
        matches!(self.ty, IpType::V4)
    }

    pub fn is_v6(&self) -> bool {
        matches!(self.ty, IpType::V6)
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.data[0] == 0
            && self.data[1] == 0
            && self.data[2] == 0
            && self.data[3] == 0
            && self.port == 0
    }
}

#[repr(u16)]
#[derive(StrEnum, Debug)]
#[allow(non_camel_case_types)]
pub enum SaFamily {
    /* Supported address families. */
    AF_UNSPEC = 0,
    AF_UNIX = 1, /* Unix domain sockets 		*/
    //AF_LOCAL=1	,/* POSIX name for AF_UNIX	*/
    AF_INET = 2,      /* Internet IP Protocol 	*/
    AF_AX25 = 3,      /* Amateur Radio AX.25 		*/
    AF_IPX = 4,       /* Novell IPX 			*/
    AF_APPLETALK = 5, /* AppleTalk DDP 		*/
    AF_NETROM = 6,    /* Amateur Radio NET/ROM 	*/
    AF_BRIDGE = 7,    /* Multiprotocol bridge 	*/
    AF_ATMPVC = 8,    /* ATM PVCs			*/
    AF_X25 = 9,       /* Reserved for X.25 project 	*/
    AF_INET6 = 10,    /* IP version 6			*/
    AF_ROSE = 11,     /* Amateur Radio X.25 PLP	*/
    AF_DECnet = 12,   /* Reserved for DECnet project	*/
    AF_NETBEUI = 13,  /* Reserved for 802.2LLC project*/
    AF_SECURITY = 14, /* Security callback pseudo AF */
    AF_KEY = 15,      /* PF_KEY key management API */
    AF_NETLINK = 16,
    //AF_ROUTE=AF_NETLINK ,/* Alias to emulate 4.4BSD */
    AF_PACKET = 17,     /* Packet family		*/
    AF_ASH = 18,        /* Ash				*/
    AF_ECONET = 19,     /* Acorn Econet			*/
    AF_ATMSVC = 20,     /* ATM SVCs			*/
    AF_RDS = 21,        /* RDS sockets 			*/
    AF_SNA = 22,        /* Linux SNA Project (nutters!) */
    AF_IRDA = 23,       /* IRDA sockets			*/
    AF_PPPOX = 24,      /* PPPoX sockets		*/
    AF_WANPIPE = 25,    /* Wanpipe API Sockets */
    AF_LLC = 26,        /* Linux LLC			*/
    AF_IB = 27,         /* Native InfiniBand address	*/
    AF_MPLS = 28,       /* MPLS */
    AF_CAN = 29,        /* Controller Area Network      */
    AF_TIPC = 30,       /* TIPC sockets			*/
    AF_BLUETOOTH = 31,  /* Bluetooth sockets 		*/
    AF_IUCV = 32,       /* IUCV sockets			*/
    AF_RXRPC = 33,      /* RxRPC sockets 		*/
    AF_ISDN = 34,       /* mISDN sockets 		*/
    AF_PHONET = 35,     /* Phonet sockets		*/
    AF_IEEE802154 = 36, /* IEEE802154 sockets		*/
    AF_CAIF = 37,       /* CAIF sockets			*/
    AF_ALG = 38,        /* Algorithm sockets		*/
    AF_NFC = 39,        /* NFC sockets			*/
    AF_VSOCK = 40,      /* vSockets			*/
    AF_KCM = 41,        /* Kernel Connection Multiplexor*/
    AF_QIPCRTR = 42,    /* Qualcomm IPC Router          */
    AF_SMC = 43,        /* smc sockets: reserve number for
                        PF_SMC protocol family that
                        reuses AF_INET address family
                        */
    AF_XDP = 44, /* XDP sockets*/
}

impl SaFamily {
    pub fn is_valid_sa_family<T: Into<u64>>(sa_family: T) -> bool {
        Self::try_from_uint(sa_family).is_ok()
    }
}

#[repr(u16)]
#[derive(StrEnum, Debug, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum SockType {
    SOCK_STREAM = 1,
    SOCK_DGRAM = 2,
    SOCK_RAW = 3,
    SOCK_RDM = 4,
    SOCK_SEQPACKET = 5,
    SOCK_DCCP = 6,
    SOCK_PACKET = 10,
}

impl SockType {
    pub fn is_valid_type<T: Into<u64>>(ty: T) -> bool {
        Self::try_from_uint(ty).is_ok()
    }
}

// IPPROTO_ macros defined in the Linux kernel
// even though some IPPROTO_ are u16 those can
// be casted to their u8 counterpart probably
// because in IP header the protocol is u8
// https://elixir.bootlin.com/linux/v6.11/source/include/uapi/linux/in.h#L29
// https://elixir.bootlin.com/linux/v6.11/source/include/uapi/linux/in6.h#L132
#[repr(u16)]
#[derive(StrEnum, Debug, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum IpProto {
    IP = 0,         /* Dummy protocol for TCP		*/
    ICMP = 1,       /* Internet Control Message Protocol	*/
    IGMP = 2,       /* Internet Group Management Protocol	*/
    IPIP = 4,       /* IPIP tunnels (older KA9Q tunnels use 94) */
    TCP = 6,        /* Transmission Control Protocol	*/
    EGP = 8,        /* Exterior Gateway Protocol		*/
    PUP = 12,       /* PUP protocol				*/
    UDP = 17,       /* User Datagram Protocol		*/
    IDP = 22,       /* XNS IDP protocol			*/
    TP = 29,        /* SO Transport Protocol Class 4	*/
    DCCP = 33,      /* Datagram Congestion Control Protocol */
    IPV6 = 41,      /* IPv6-in-IPv4 tunnelling		*/
    RSVP = 46,      /* RSVP Protocol			*/
    GRE = 47,       /* Cisco GRE tunnels (rfc 1701,1702)	*/
    ESP = 50,       /* Encapsulation Security Payload protocol */
    AH = 51,        /* Authentication Header protocol	*/
    MTP = 92,       /* Multicast Transport Protocol		*/
    BEETPH = 94,    /* IP option pseudo header for BEET	*/
    ENCAP = 98,     /* Encapsulation Header			*/
    PIM = 103,      /* Protocol Independent Multicast	*/
    COMP = 108,     /* Compression Header Protocol		*/
    L2TP = 115,     /* Layer 2 Tunnelling Protocol		*/
    SCTP = 132,     /* Stream Control Transport Protocol	*/
    UDPLITE = 136,  /* UDP-Lite (RFC 3828)			*/
    MPLS = 137,     /* MPLS in IP (RFC 4023)		*/
    ETHERNET = 143, /* Ethernet-within-IPv6 Encapsulation	*/
    RAW = 255,      /* Raw IP packets			*/
    SMC = 256,      /* Shared Memory Communications		*/
    MPTCP = 262,    /* Multipath TCP connection		*/
    // IPv6 related
    ROUTING = 43,  /* IPv6 routing header		*/
    FRAGMENT = 44, /* IPv6 fragmentation header	*/
    ICMPV6 = 58,   /* ICMPv6			*/
    NONE = 59,     /* IPv6 no next header		*/
    DSTOPTS = 60,  /* IPv6 destination options	*/
    MH = 135,      /* IPv6 mobility header		*/
}

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct SocketInfo {
    /// Must be [SaFamily] value
    pub domain: u16,
    /// Must be [SockType] value
    pub ty: u16,
    /// Value of socket.sk_protocol
    pub proto: u16,
}

impl SocketInfo {
    #[inline(always)]
    pub fn is_family(&self, sa: SaFamily) -> bool {
        self.domain == sa as u16
    }

    #[inline(always)]
    pub fn is_type(&self, ty: SockType) -> bool {
        self.ty == ty as u16
    }
}
