# *** Generated by ConfigerIO ***
# {{{timestamp}}}
# {{{hostname}}}

server {
	listen *:80;
	server_name {{{hostname}}};
	root /home/{{{user}}}/www/;
	index index.php index.html index.htm;
	location / {
		try_files $uri $uri/ /index.php;
		autoindex on;
		autoindex_exact_size off;
	}
#	location ~ \.php$ {
#		fastcgi_pass unix:/run/php-{{{user}}}.sock;
#		fastcgi_index index.php;
#		fastcgi_param SCRIPT_FILENAME $request_filename;
#		include /etc/nginx/fastcgi_params;
#	}
}
