use common::{Name, Attribute, XmlVersion};
use namespace::Namespace;

/// An element of an XML output stream.
///
/// Items of this enum are consumed by `writer::EventWriter`. They correspond to different
/// elements of an XML document.
#[deriving(PartialEq, Clone)]
pub enum XmlEvent<'a> {
    /// Corresponds to XML document declaration. 
    ///
    /// This event should always be written before any other event. If it is not written
    /// at all, default XML declaration will be outputted.
    StartDocument {
        /// XML version.
        ///
        /// If XML declaration is not present, defaults to `Version10`.
        pub version: XmlVersion,

        /// XML document encoding.
        pub encoding: Option<&'a str>,

        /// XML standalone declaration.
        pub standalone: Option<bool>
    },

    /// Denotes to the end of the document stream.
    EndDocument,

    /// Denotes an XML processing instruction.
    ///
    /// This event contains a processing instruction target (`name`) and opaque `data`. It
    /// is up to the application to process them.
    ProcessingInstruction { 
        /// Processing instruction target.
        pub name: &'a str, 

        /// Processing instruction content.
        pub data: Option<&'a str> 
    },

    /// Denotes a beginning of an XML element.
    ///
    /// This event is emitted after parsing opening tags or after parsing bodiless tags. In the
    /// latter case `EndElement` event immediately follows.
    StartElement { 
        /// Qualified name of the element.
        pub name: Name,

        /// A list of attributes associated with the element.
        /// 
        /// Currently attributes are not checked for duplicates (TODO)
        pub attributes: Vec<Attribute>,

        /// Contents of the namespace mapping at this point of the document.
        pub namespace: Namespace,
    },

    /// Denotes an end of an XML document.
    ///
    /// This event is emitted after parsing closing tags or after parsing bodiless tags. In the
    /// latter case it is emitted immediately after corresponding `StartElement` event.
    EndElement {
        /// Qualified name of the element.
        pub name: Name
    },

    /// Denotes CDATA content.
    ///
    /// This event contains unparsed data. No unescaping will be performed.
    ///
    /// It is possible to configure a parser to emit `Characters` event instead of `CData`. See
    /// `reader::ParserConfiguration` structure for more information.
    CData(&'a str),

    /// Denotes a comment.
    ///
    /// It is possible to configure a parser to ignore comments, so this event will never be emitted.
    /// See `reader::ParserConfiguration` structure for more information.
    Comment(&'a str),

    /// Denotes character data outside of tags.
    ///
    /// Contents of this event will always be unescaped, so no entities like `&lt;` or `&amp;` or `&#123;`
    /// will appear in it.
    ///
    /// It is possible to configure a parser to trim leading and trailing whitespace for this event.
    /// See `reaer::ParserConfiguration` structure for more information.
    Characters(&'a str),

    /// Denotes a chunk of whitespace outside of tags.
    ///
    /// It is possible to configure a parser to emit `Characters` event instead of `Whitespace`.
    /// See `reader::ParserConfiguration` structure for more information. When combined with whitespace
    /// trimming, it will eliminate standalone whitespace from the event stream completely.
    Whitespace(&'a str)
}
