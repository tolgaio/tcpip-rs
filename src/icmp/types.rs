#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum ICMPType {
    DestinationUnreachable = 1,
    PacketTooBig = 2,
    TimeExceeded = 3,
    ParameterProblem = 4,
    EchoRequest = 128,
    EchoReply = 129,
    MulticastListenerQuery = 130,
    MulticastListenerReport = 131,
    MulticastListenerDone = 132,
    RouterSolicitation = 133,
    RouterAdvertisement = 134,
    NeighborSolicitation = 135,
    NeighborAdvertisement = 136,
    RedirectMessage = 137,
    RouterRenumbering = 138,
    ICMPNodeInformationQuery = 139,
    ICMPNodeInformationResponse = 140,
    InverseNeighborDiscoverySolicitationMessage = 141,
    InverseNeighborDiscoveryAdvertisementMessage = 142,
    Version2MulticastListenerReport = 143,
    HomeAgentAddressDiscoveryRequestMessage = 144,
    HomeAgentAddressDiscoveryReplyMessage = 145,
    MobilePrefixSolicitation = 146,
    MobilePrefixAdvertisement = 147,
    CertificationPathSolicitationMessage = 148,
    CertificationPathAdvertisementMessage = 149,
    MulticastRouterAdvertisement = 151,
    MulticastRouterSolicitation = 152,
    MulticastRouterTermination = 153,
    FMIPv6Messages = 154,
    RPLControlMessage = 155,
    ILNPv6LocatorUpdateMessage = 156,
    DuplicateAddressRequest = 157,
    DuplicateAddressConfirmation = 158,
    MPLControlMessage = 159,
    ExtendedEchoRequest = 160,
    ExtendedEchoReply = 161,
    Unknown = -1,
}

impl From<u8> for ICMPType {
    fn from(orig: u8) -> Self {
        match orig {
            1 => ICMPType::DestinationUnreachable,
            2 => ICMPType::PacketTooBig,
            3 => ICMPType::TimeExceeded,
            4 => ICMPType::ParameterProblem,
            128 => ICMPType::EchoRequest,
            129 => ICMPType::EchoReply,
            130 => ICMPType::MulticastListenerQuery,
            131 => ICMPType::MulticastListenerReport,
            132 => ICMPType::MulticastListenerDone,
            133 => ICMPType::RouterSolicitation,
            134 => ICMPType::RouterAdvertisement,
            135 => ICMPType::NeighborSolicitation,
            136 => ICMPType::NeighborAdvertisement,
            137 => ICMPType::RedirectMessage,
            138 => ICMPType::RouterRenumbering,
            139 => ICMPType::ICMPNodeInformationQuery,
            140 => ICMPType::ICMPNodeInformationResponse,
            141 => ICMPType::InverseNeighborDiscoverySolicitationMessage,
            142 => ICMPType::InverseNeighborDiscoveryAdvertisementMessage,
            143 => ICMPType::Version2MulticastListenerReport,
            144 => ICMPType::HomeAgentAddressDiscoveryRequestMessage,
            145 => ICMPType::HomeAgentAddressDiscoveryReplyMessage,
            146 => ICMPType::MobilePrefixSolicitation,
            147 => ICMPType::MobilePrefixAdvertisement,
            148 => ICMPType::CertificationPathSolicitationMessage,
            149 => ICMPType::CertificationPathAdvertisementMessage,
            151 => ICMPType::MulticastRouterAdvertisement,
            152 => ICMPType::MulticastRouterSolicitation,
            153 => ICMPType::MulticastRouterTermination,
            154 => ICMPType::FMIPv6Messages,
            155 => ICMPType::RPLControlMessage,
            156 => ICMPType::ILNPv6LocatorUpdateMessage,
            157 => ICMPType::DuplicateAddressRequest,
            158 => ICMPType::DuplicateAddressConfirmation,
            159 => ICMPType::MPLControlMessage,
            160 => ICMPType::ExtendedEchoRequest,
            161 => ICMPType::ExtendedEchoReply,
            _ => ICMPType::Unknown,
        }
    }
}
