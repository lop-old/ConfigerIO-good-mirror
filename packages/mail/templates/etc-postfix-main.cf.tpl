# *** Generated by ConfigerIO ***
# {{{timestamp}}}
#
# <don't edit here; changes will be overwritten>

compatibility_level = 2
#soft_bounce = no
queue_directory = /var/spool/postfix
command_directory = /usr/sbin
daemon_directory = /usr/libexec/postfix
data_directory = /var/lib/postfix
mail_owner = postfix
#default_privs = nobody
#myhostname = host.domain.tld
#myhostname = virtual.domain.tld
#mydomain = domain.tld

#myorigin = $myhostname
#myorigin = $mydomain

inet_interfaces = all
inet_protocols = all

mydestination = $myhostname, localhost.$mydomain, localhost

unknown_local_recipient_reject_code = 550

alias_maps = hash:/etc/aliases
alias_database = hash:/etc/aliases

home_mailbox = .mail
mail_spool_directory = /var/mail

debug_peer_level = 2
#debug_peer_list = 127.0.0.1
#debug_peer_list = some.domain

#debugger_command =
#	 PATH=/bin:/usr/bin:/usr/local/bin:/usr/X11R6/bin
#	 ddd $daemon_directory/$process_name $process_id & sleep 5

sendmail_path = /usr/sbin/sendmail.postfix
newaliases_path = /usr/bin/newaliases.postfix
mailq_path = /usr/bin/mailq.postfix
setgid_group = postdrop

html_directory = no

smtpd_tls_cert_file = /etc/pki/tls/certs/postfix.pem
smtpd_tls_key_file = /etc/pki/tls/private/postfix.key
smtpd_tls_security_level = may
smtp_tls_CApath = /etc/pki/tls/certs
smtp_tls_CAfile = /etc/pki/tls/certs/ca-bundle.crt

smtp_tls_security_level = may
meta_directory = /etc/postfix
shlib_directory = /usr/lib64/postfix
