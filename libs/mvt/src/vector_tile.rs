#[derive(Clone, PartialEq, Message)]
pub struct Tile {
    #[prost(message, repeated, tag="3")]
    pub layers: ::std::vec::Vec<tile::Layer>,
}
pub mod tile {
    /// Variant type encoding
    /// The use of values is described in section 4.1 of the specification
    #[derive(Clone, PartialEq, Message)]
    pub struct Value {
        /// Exactly one of these values must be present in a valid message
        #[prost(string, optional, tag="1")]
        pub string_value: ::std::option::Option<String>,
        #[prost(float, optional, tag="2")]
        pub float_value: ::std::option::Option<f32>,
        #[prost(double, optional, tag="3")]
        pub double_value: ::std::option::Option<f64>,
        #[prost(int64, optional, tag="4")]
        pub int_value: ::std::option::Option<i64>,
        #[prost(uint64, optional, tag="5")]
        pub uint_value: ::std::option::Option<u64>,
        #[prost(sint64, optional, tag="6")]
        pub sint_value: ::std::option::Option<i64>,
        #[prost(bool, optional, tag="7")]
        pub bool_value: ::std::option::Option<bool>,
    }
    /// Features are described in section 4.2 of the specification
    #[derive(Clone, PartialEq, Message)]
    pub struct Feature {
        #[prost(uint64, optional, tag="1", default="0")]
        pub id: ::std::option::Option<u64>,
        /// Tags of this feature are encoded as repeated pairs of
        /// integers.
        /// A detailed description of tags is located in sections
        /// 4.2 and 4.4 of the specification
        #[prost(uint32, repeated, tag="2")]
        pub tags: ::std::vec::Vec<u32>,
        /// The type of geometry stored in this feature.
        #[prost(enumeration="GeomType", optional, tag="3", default="Unknown")]
        pub type_: ::std::option::Option<i32>,
        /// Contains a stream of commands and parameters (vertices).
        /// A detailed description on geometry encoding is located in 
        /// section 4.3 of the specification.
        #[prost(uint32, repeated, tag="4")]
        pub geometry: ::std::vec::Vec<u32>,
    }
    /// Layers are described in section 4.1 of the specification
    #[derive(Clone, PartialEq, Message)]
    pub struct Layer {
        /// Any compliant implementation must first read the version
        /// number encoded in this message and choose the correct
        /// implementation for this version number before proceeding to
        /// decode other parts of this message.
        #[prost(uint32, required, tag="15", default="1")]
        pub version: u32,
        #[prost(string, required, tag="1")]
        pub name: String,
        /// The actual features in this tile.
        #[prost(message, repeated, tag="2")]
        pub features: ::std::vec::Vec<Feature>,
        /// Dictionary encoding for keys
        #[prost(string, repeated, tag="3")]
        pub keys: ::std::vec::Vec<String>,
        /// Dictionary encoding for values
        #[prost(message, repeated, tag="4")]
        pub values: ::std::vec::Vec<Value>,
        /// Although this is an "optional" field it is required by the specification.
        /// See https://github.com/mapbox/vector-tile-spec/issues/47
        #[prost(uint32, optional, tag="5", default="4096")]
        pub extent: ::std::option::Option<u32>,
    }
    /// GeomType is described in section 4.3.4 of the specification
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Enumeration)]
    pub enum GeomType {
        Unknown = 0,
        Point = 1,
        Linestring = 2,
        Polygon = 3,
    }

    impl ToString for GeomType {
        fn to_string(&self) -> String {
            match self {
                GeomType::Unknown => "Unknown",
                GeomType::Point => "Point",
                GeomType::Linestring => "LineString",
                GeomType::Polygon => "Polygon",
            }.into()
        }
    }
}
