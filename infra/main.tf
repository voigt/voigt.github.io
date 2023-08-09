terraform {
  required_providers {
    cloudflare = {
      source  = "cloudflare/cloudflare"
      version = "~> 4.0"
    }
  }
}

variable "domain" {
  default = "christophvoigt.com"
}

variable "zone_id" {
  default = "5135e0adad7dabaa3699abac5bba8559"
}

resource "cloudflare_record" "www" {
  zone_id = "${var.zone_id}"
  name    = "www"
  value   = "christophvoigt.com"
  type    = "CNAME"
  ttl     = 1
}

resource "cloudflare_record" "a_record_github_one" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "185.199.111.153"
  type    = "A"
  ttl     = 1
}

resource "cloudflare_record" "a_record_github_two" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "185.199.110.153"
  type    = "A"
  ttl     = 1
}

resource "cloudflare_record" "a_record_github_three" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "185.199.109.153"
  type    = "A"
  ttl     = 1
}

resource "cloudflare_record" "a_record_github_four" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "185.199.108.153"
  type    = "A"
  ttl     = 1
}

resource "cloudflare_record" "aaaa_record_github_one" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "2606:50c0:8003::153"
  type    = "AAAA"
  ttl     = 1
}

resource "cloudflare_record" "aaaa_record_github_two" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "2606:50c0:8002::153"
  type    = "AAAA"
  ttl     = 1
}

resource "cloudflare_record" "aaaa_record_github_three" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "2606:50c0:8001::153"
  type    = "AAAA"
  ttl     = 1
}

resource "cloudflare_record" "aaaa_record_github_four" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "2606:50c0:8000::153"
  type    = "AAAA"
  ttl     = 1
}

resource "cloudflare_record" "ns_record_fermyon_one" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "dns1.p05.nsone.net"
  type    = "NS"
  ttl     = 1
}

resource "cloudflare_record" "ns_record_fermyon_two" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "dns2.p05.nsone.net"
  type    = "NS"
  ttl     = 1
}

resource "cloudflare_record" "ns_record_fermyon_three" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "dns3.p05.nsone.net"
  type    = "NS"
  ttl     = 1
}

resource "cloudflare_record" "ns_record_fermyon_four" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "dns4.p05.nsone.net"
  type    = "NS"
  ttl     = 1
}

resource "cloudflare_record" "txt_record_one" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "v=spf1 -all"
  type    = "TXT"
  ttl     = 1
}

resource "cloudflare_record" "txt_record_two" {
  zone_id = "${var.zone_id}"
  name    = "${var.domain}"
  value   = "keybase-site-verification=zIdk4JdfmTVd7b7hW2wCGANSaSO8oaN62ds-4Et8Cm0"
  type    = "TXT"
  ttl     = 1
}

resource "cloudflare_record" "txt_record_three" {
  zone_id = "${var.zone_id}"
  name    = "_dmarc"
  value   = "v=DMARC1; p=reject; sp=reject; adkim=s; aspf=s;"
  type    = "TXT"
  ttl     = 1
}

resource "cloudflare_record" "txt_record_four" {
  zone_id = "${var.zone_id}"
  name    = "*._domainkey"
  value   = "v=DKIM1; p="
  type    = "TXT"
  ttl     = 1
}
