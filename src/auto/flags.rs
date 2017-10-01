// This file was generated by gir (0fe730d) from gir-files (db49619)
// DO NOT EDIT

use ffi;
use glib::Type;
use glib::StaticType;
use glib::value::{Value, SetValue, FromValue, FromValueOptional};
use gobject_ffi;
use glib::translate::*;

bitflags! {
    pub struct AppInfoCreateFlags: u32 {
        const NONE = 0;
        const NEEDS_TERMINAL = 1;
        const SUPPORTS_URIS = 2;
        const SUPPORTS_STARTUP_NOTIFICATION = 4;
    }
}

#[doc(hidden)]
impl ToGlib for AppInfoCreateFlags {
    type GlibType = ffi::GAppInfoCreateFlags;

    fn to_glib(&self) -> ffi::GAppInfoCreateFlags {
        ffi::GAppInfoCreateFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GAppInfoCreateFlags> for AppInfoCreateFlags {
    fn from_glib(value: ffi::GAppInfoCreateFlags) -> AppInfoCreateFlags {
        AppInfoCreateFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for AppInfoCreateFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_app_info_create_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for AppInfoCreateFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for AppInfoCreateFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GAppInfoCreateFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for AppInfoCreateFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct ApplicationFlags: u32 {
        const FLAGS_NONE = 0;
        const IS_SERVICE = 1;
        const IS_LAUNCHER = 2;
        const HANDLES_OPEN = 4;
        const HANDLES_COMMAND_LINE = 8;
        const SEND_ENVIRONMENT = 16;
        const NON_UNIQUE = 32;
        const CAN_OVERRIDE_APP_ID = 64;
    }
}

#[doc(hidden)]
impl ToGlib for ApplicationFlags {
    type GlibType = ffi::GApplicationFlags;

    fn to_glib(&self) -> ffi::GApplicationFlags {
        ffi::GApplicationFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GApplicationFlags> for ApplicationFlags {
    fn from_glib(value: ffi::GApplicationFlags) -> ApplicationFlags {
        ApplicationFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for ApplicationFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_application_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ApplicationFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ApplicationFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GApplicationFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for ApplicationFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct FileCreateFlags: u32 {
        const NONE = 0;
        const PRIVATE = 1;
        const REPLACE_DESTINATION = 2;
    }
}

#[doc(hidden)]
impl ToGlib for FileCreateFlags {
    type GlibType = ffi::GFileCreateFlags;

    fn to_glib(&self) -> ffi::GFileCreateFlags {
        ffi::GFileCreateFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GFileCreateFlags> for FileCreateFlags {
    fn from_glib(value: ffi::GFileCreateFlags) -> FileCreateFlags {
        FileCreateFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for FileCreateFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_file_create_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FileCreateFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FileCreateFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GFileCreateFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for FileCreateFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct FileQueryInfoFlags: u32 {
        const NONE = 0;
        const NOFOLLOW_SYMLINKS = 1;
    }
}

#[doc(hidden)]
impl ToGlib for FileQueryInfoFlags {
    type GlibType = ffi::GFileQueryInfoFlags;

    fn to_glib(&self) -> ffi::GFileQueryInfoFlags {
        ffi::GFileQueryInfoFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GFileQueryInfoFlags> for FileQueryInfoFlags {
    fn from_glib(value: ffi::GFileQueryInfoFlags) -> FileQueryInfoFlags {
        FileQueryInfoFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for FileQueryInfoFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_file_query_info_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for FileQueryInfoFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for FileQueryInfoFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GFileQueryInfoFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for FileQueryInfoFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct ResourceLookupFlags: u32 {
        const NONE = 0;
    }
}

#[doc(hidden)]
impl ToGlib for ResourceLookupFlags {
    type GlibType = ffi::GResourceLookupFlags;

    fn to_glib(&self) -> ffi::GResourceLookupFlags {
        ffi::GResourceLookupFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GResourceLookupFlags> for ResourceLookupFlags {
    fn from_glib(value: ffi::GResourceLookupFlags) -> ResourceLookupFlags {
        ResourceLookupFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for ResourceLookupFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_resource_lookup_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for ResourceLookupFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for ResourceLookupFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GResourceLookupFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for ResourceLookupFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct SettingsBindFlags: u32 {
        const DEFAULT = 0;
        const GET = 1;
        const SET = 2;
        const NO_SENSITIVITY = 4;
        const GET_NO_CHANGES = 8;
        const INVERT_BOOLEAN = 16;
    }
}

#[doc(hidden)]
impl ToGlib for SettingsBindFlags {
    type GlibType = ffi::GSettingsBindFlags;

    fn to_glib(&self) -> ffi::GSettingsBindFlags {
        ffi::GSettingsBindFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GSettingsBindFlags> for SettingsBindFlags {
    fn from_glib(value: ffi::GSettingsBindFlags) -> SettingsBindFlags {
        SettingsBindFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for SettingsBindFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_settings_bind_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for SettingsBindFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for SettingsBindFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GSettingsBindFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for SettingsBindFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

bitflags! {
    pub struct TlsCertificateFlags: u32 {
        const UNKNOWN_CA = 1;
        const BAD_IDENTITY = 2;
        const NOT_ACTIVATED = 4;
        const EXPIRED = 8;
        const REVOKED = 16;
        const INSECURE = 32;
        const GENERIC_ERROR = 64;
        const VALIDATE_ALL = 127;
    }
}

#[doc(hidden)]
impl ToGlib for TlsCertificateFlags {
    type GlibType = ffi::GTlsCertificateFlags;

    fn to_glib(&self) -> ffi::GTlsCertificateFlags {
        ffi::GTlsCertificateFlags::from_bits_truncate(self.bits())
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GTlsCertificateFlags> for TlsCertificateFlags {
    fn from_glib(value: ffi::GTlsCertificateFlags) -> TlsCertificateFlags {
        TlsCertificateFlags::from_bits_truncate(value.bits())
    }
}

impl StaticType for TlsCertificateFlags {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_tls_certificate_flags_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for TlsCertificateFlags {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for TlsCertificateFlags {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(ffi::GTlsCertificateFlags::from_bits_truncate(gobject_ffi::g_value_get_flags(value.to_glib_none().0)))
    }
}

impl SetValue for TlsCertificateFlags {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_flags(value.to_glib_none_mut().0, this.to_glib().bits())
    }
}

