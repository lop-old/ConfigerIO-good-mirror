# Generated: Thu Jun  2 07:15:02 AM EDT 2022
Name    : configer-php
Version : 0.1.%{?build_number}%{!?build_number:x}
Release : 1%{dist}
Summary : PHP-FPM module for ConfigerIO

Requires: php
BuildRequires: systemd-rpm-macros

BuildArch : x86_64
Packager  : PoiXson <support@poixson.com>
License   : GPLv3
URL       : https://configer.io

Prefix: %{_datadir}/configer
%define _rpmfilename  %%{NAME}-%%{VERSION}-%%{RELEASE}.%%{ARCH}.rpm

%description
PHP-FPM module for ConfigerIO



### Install ###
%install
echo
echo "Install.."

# delete existing rpm's
%{__rm} -fv --preserve-root  "%{_rpmdir}/%{name}-"*.rpm

# create dirs
%{__install} -d -m 0755  \
	"%{buildroot}%{_bindir}/"                     \
	"%{buildroot}%{_sysconfdir}/systemd/system/"  \
	"%{buildroot}%{_presetdir}/"                  \
	"%{buildroot}%{prefix}/"                      \
	"%{buildroot}%{prefix}/templates/php/"        \
		|| exit 1

# copy files
\pushd "%{_topdir}/../" >/dev/null || exit 1
	if [[ -d target/release/ ]]; then
		%{__install} -m 0755  "target/release/configer-php"  "%{buildroot}%{_bindir}/"  || exit 1
	else
		%{__install} -m 0755  "target/debug/configer-php"    "%{buildroot}%{_bindir}/"  || exit 1
	fi
	# templates
	%{__install} -m 0755  "templates/"*.tpl  "%{buildroot}%{prefix}/templates/php/"  || exit 1
	# systemd
	%{__install} -m 0755  "php-fpm.service"  "%{buildroot}%{_sysconfdir}/systemd/system/"  || exit 1
	%{__install} -m 0755  "10-php-fpm.preset"  "%{buildroot}%{_presetdir}/"                || exit 1
\popd >/dev/null



%post
%systemd_post  php-fpm.service
if [[ "$1" -eq 1 ]]; then
	/usr/bin/systemctl start  php-fpm.service  || :
fi

%preun
%systemd_preun  php-fpm.service

%postun
%systemd_postun_with_restart  php-fpm.service



### Files ###
%files
%defattr(0400, configer, configer, 0500)
%attr(0500,root,root)  %{_bindir}/configer-php
%{_sysconfdir}/systemd/system/php-fpm.service
%{_presetdir}/10-php-fpm.preset
%{prefix}/templates/php/etc-php.ini.tpl
%{prefix}/templates/php/etc-php-fpm.conf.tpl
%{prefix}/templates/php/etc-php-fpm.d-user.conf.tpl
