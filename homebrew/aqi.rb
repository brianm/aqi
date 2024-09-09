require "formula"

$pkg_version = "0.2.1"
$pkg_group = "brianm"
$pkg_repo = "aqi"
$pkg_tag = "#{$pkg_repo}-#{$pkg_version}"

class Aqi < Formula
  head "https://github.com/#{$pkg_group}/#{$pkg_repo}.git"

  url "https://github.com/#{$pkg_group}/#{$pkg_repo}/archive/#{$pkg_tag}.tar.gz"
  version $pkg_version
  homepage "https://github.com/#{$pkg_group}/#{$pkg_repo}"
  sha256 "192f07cdb72193a1004631d73fbfe053635f1a2ae7d5ecae891ec74e0545c4b6"

  depends_on "rust" => :build
  depends_on "openssl"

  def install
    system "cargo build --release"
    bin.install "target/release/aqi"
  end
end
