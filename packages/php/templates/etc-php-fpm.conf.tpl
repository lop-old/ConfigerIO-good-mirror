; *** Generated by ConfigerIO ***
; {{{timestamp}}}
;
; <don't edit here; changes will be overwritten>

[global]
pid = /run/php-fpm.pid
daemonize = no

error_log = /var/log/php-fpm/error.log
log_level = notice

;emergency_restart_threshold = 0
;emergency_restart_interval = 0
;process_control_timeout = 0
;process.max = 128
;process.priority = -19
;rlimit_files = 1024
;rlimit_core = 0
;events.mechanism = epoll
;systemd_interval = 10

catch_workers_output = yes
request_terminate_timeout = 120s


include=/etc/php-fpm.d/*.conf
