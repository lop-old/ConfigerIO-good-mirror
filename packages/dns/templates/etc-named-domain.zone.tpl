; *** Generated by ConfigerIO ***
; {{{timestamp}}}
$ORIGIN {{{domain}}}.
$TTL 2D
@   IN   SOA   ns1 support (
               2022040300 ; serial number
               12h        ; refresh
               15m        ; refresh retry
               3w         ; expiry
               2h         ; nxdomain ttl
               )
{{{domain}}}.  IN  NS  ns1.{{{domain}}}.
{{{domain}}}.  IN  NS  ns2.{{{domain}}}.

ns1  IN  A  {{{ip}}}
ns2  IN  A  {{{ip}}}

{{{domain}}}.  IN  A   {{{ip}}}
{{{domain}}}.  IN  MX  0 {{{domain}}}.

www   IN  CNAME  {{{domain}}}.
mail  IN  CNAME  {{{domain}}}.