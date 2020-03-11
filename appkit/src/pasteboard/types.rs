//! This module provides some basic wrappers for Pasteboard functionality. It's currently not an
//! exhaustive clone, but feel free to pull request accordingly!

use cocoa::base::{id, nil};
use cocoa::foundation::NSString;

/// Constants for the standard system pasteboard names.
#[derive(Debug, Copy, Clone)]
pub enum PasteboardName {
    /// The dragging/dropping pasteboard.
    Drag,

    /// The find pasteboard.
    Find,

    /// The font pasteboard.
    Font,

    /// The general pasteboard.
    General,

    /// The ruler pasteboard.
    Ruler
}

impl PasteboardName {
    /// Creates an `NSString` out of the underlying type.
    pub fn to_nsstring(&self) -> id {
        unsafe {
            NSString::alloc(nil).init_str(match self {
                PasteboardName::Drag => "Apple CFPasteboard drag",
                PasteboardName::Find => "Apple CFPasteboard find",
                PasteboardName::Font => "Apple CFPasteboard font",
                PasteboardName::General => "Apple CFPasteboard general",
                PasteboardName::Ruler => "Apple CFPasteboard ruler"
            })
        }
    }
}

/// Represents different Pasteboard types that can be referred to.
#[derive(Debug, Copy, Clone)]
pub enum PasteboardType {
    /// URL data for one file or resource.
    URL,

    /// Color data.
    Color,

    /// A file URL.
    FileURL,

    /// Font and character information.
    Font,

    /// Type for HTML content.
    HTML,

    /// Multiple text selection.
    MultipleTextSelection,

    /// PDF data.
    PDF,

    /// PNG image data.
    PNG,

    /// Rich Text Format (RTF) data.
    RTF,

    /// RTFD formatted file contents.
    RTFD,

    /// Paragraph formatting information.
    Ruler,

    /// Sound data.
    Sound,

    /// String data.
    String,

    /// Tab-separated fields of text.
    TabularText,

    /// Tag Image File Format (TIFF) data.
    TIFF
}

impl PasteboardType {
    /// Creates an `NSString` out of the underlying type.
    pub fn to_nsstring(&self) -> id {
        unsafe {
            NSString::alloc(nil).init_str(match self {
                PasteboardType::URL => "public.url",
                PasteboardType::Color => "com.apple.cocoa.pasteboard.color",
                PasteboardType::FileURL => "public.file-url",
                PasteboardType::Font => "com.apple.cocoa.pasteboard.character-formatting",
                PasteboardType::HTML => "public.html",
                PasteboardType::MultipleTextSelection => "com.apple.cocoa.pasteboard.multiple-text-selection",
                PasteboardType::PDF => "com.adobe.pdf",
                PasteboardType::PNG => "public.png",
                PasteboardType::RTF => "public.rtf",
                PasteboardType::RTFD => "com.apple.flat-rtfd",
                PasteboardType::Ruler => "com.apple.cocoa.pasteboard.paragraph-formatting",
                PasteboardType::Sound => "com.apple.cocoa.pasteboard.sound",
                PasteboardType::String => "public.utf8-plain-text",
                PasteboardType::TabularText => "public.utf8-tab-separated-values-text",
                PasteboardType::TIFF => "public.tiff",
            })
        }
    }
}
