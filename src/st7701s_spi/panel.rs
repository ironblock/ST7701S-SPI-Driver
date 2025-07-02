pub struct Mode {
  pub clock: u32,

  pub hdisplay: u32,
  pub hsync_start: u32,
  pub hsync_end: u32,
  pub htotal: u32,

  pub vdisplay: u32,
  pub vsync_start: u32,
  pub vsync_end: u32,
  pub vtotal: u32,

  pub width_mm: u32,
  pub height_mm: u32,
}

pub const DefaultMode: Mode = Mode {
  clock: 27500,

  hdisplay: 480,
  hsync_start: 480 + 38,
  hsync_end: 480 + 38 + 12,
  htotal: 480 + 38 + 12 + 12,

  vdisplay: 854,
  vsync_start: 854 + 18,
  vsync_end: 854 + 18 + 8,
  vtotal: 854 + 18 + 8 + 4,

  width_mm: 69,
  height_mm: 139,
};

pub const TDOMode: Mode = Mode {
  clock: 16000,

  hdisplay: 480,
  hsync_start: 480 + 24,
  hsync_end: 480 + 24 + 6,
  htotal: 480 + 24 + 6 + 18,

  vdisplay: 480,
  vsync_start: 480 + 16,
  vsync_end: 480 + 16 + 4,
  vtotal: 480 + 16 + 4 + 10,

  width_mm: 69,
  height_mm: 139,
};

pub const CVTMode: Mode = Mode {
  clock: 17000,

  hdisplay: 480,
  hsync_start: 480 + 8,
  hsync_end: 480 + 8 + 48,
  htotal: 480 + 8 + 48 + 56,

  vdisplay: 480,
  vsync_start: 480 + 1,
  vsync_end: 480 + 1 + 3,
  vtotal: 480 + 1 + 3 + 13,

  width_mm: 69,
  height_mm: 139,
};

pub const CVTRB: Mode = Mode {
  clock: 23500,

  hdisplay: 640,
  hsync_start: 480 + 8,
  hsync_end: 480 + 8 + 32,
  htotal: 480 + 8 + 32 + 40,

  vdisplay: 480,
  vsync_start: 480 + 14,
  vsync_end: 480 + 14 + 3,
  vtotal: 480 + 14 + 3 + 4,

  width_mm: 69,
  height_mm: 139,
};
