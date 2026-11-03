<?php

/**
 * League.Uri (https://uri.thephpleague.com)
 *
 * (c) Ignace Nyamagana Butera <nyamsprod@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

declare(strict_types=1);

namespace League\Uri;

use ValueError;

/*
 *  Supported schemes and corresponding default port.
 *
 * @see https://github.com/python-hyper/hyperlink/blob/master/src/hyperlink/_url.py for the curating list definition
 * @see https://www.iana.org/assignments/uri-schemes/uri-schemes.xhtml
 * @see https://www.iana.org/assignments/service-names-port-numbers/service-names-port-numbers.xhtml
 */
enum UriScheme: string
{
    case About = 'about';
    case Acap = 'acap';
    case Bitcoin = 'bitcoin';
    case Geo = 'geo';
    case Blob = 'blob';
    case Afp = 'afp';
    case Data = 'data';
    case Dict = 'dict';
    case Dns = 'dns';
    case File = 'file';
    case Ftp = 'ftp';
    case Git = 'git';
    case Gopher = 'gopher';
    case Http = 'http';
    case Https = 'https';
    case Imap = 'imap';
    case Imaps = 'imaps';
    case Ipp = 'ipp';
    case Ipps = 'ipps';
    case Irc = 'irc';
    case Ircs = 'ircs';
    case Javascript = 'javascript';
    case Ldap = 'ldap';
    case Ldaps = 'ldaps';
    case Magnet = 'magnet';
    case Mailto = 'mailto';
    case Mms = 'mms';
    case Msrp = 'msrp';
    case Msrps = 'msrps';
    case Mtqp = 'mtqp';
    case News = 'news';
    case Nfs = 'nfs';
    case Nntp = 'nntp';
    case Nntps = 'nntps';
    case Pkcs11 = 'pkcs11';
    case Pop = 'pop';
    case Prospero = 'prospero';
    case Redis = 'redis';
    case Rsync = 'rsync';
    case Rtsp = 'rtsp';
    case Rtsps = 'rtsps';
    case Rtspu = 'rtspu';
    case Sftp = 'sftp';
    case Wss = 'wss';
    case Ws = 'ws';
    case Sip = 'sip';
    case Sips = 'sips';
    case Smb = 'smb';
    case Smtp = 'smtp';
    case Snmp = 'snmp';
    case Ssh = 'ssh';
    case Steam = 'steam';
    case Svn = 'svn';
    case Tel = 'tel';
    case Telnet = 'telnet';
    case Tn3270 = 'tn3270';
    case Urn = 'urn';
    case Ventrilo = 'ventrilo';
    case Vnc = 'vnc';
    case Wais = 'wais';
    case Xmpp = 'xmpp';

    public function port(): ?int
    {}

    public function type(): SchemeType
    {}

    public function isWhatWgSpecial(): bool
    {}

    /**
     * @return list<self>
     */
    public static function fromPort(?int $port): array
    {}

    public function builder(): Builder
    {}
}
