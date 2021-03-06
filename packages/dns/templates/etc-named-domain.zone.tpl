; *** Generated by ConfigerIO ***
; {{{timestamp}}}
;
; <don't edit here; changes will be overwritten>
;
$ORIGIN {{{domain}}}.
$TTL 2D
@   IN   SOA   ns1 support (
               2022040300 ; serial number
               12h        ; refresh
               15m        ; refresh retry
               3w         ; expiry
               2h         ; nxdomain ttl
               )

{{#if is_internal}}
{{{domain}}}.  IN  A   {{{details.ip}}}
{{/if}}
{{#if is_external}}
{{{domain}}}.  IN  NS  ns1.{{{domain}}}.
{{{domain}}}.  IN  NS  ns2.{{{domain}}}.
ns1  IN  A  {{{details.ip}}}
ns2  IN  A  {{{details.ip}}}

{{{domain}}}.  IN  A   {{{details.ip}}}
{{{domain}}}.  IN  MX  0 {{{domain}}}.

www   IN  CNAME  {{{domain}}}.
mail  IN  CNAME  {{{domain}}}.
{{/if}}
