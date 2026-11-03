<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\VarDumper\Caster;

use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * @author Nicolas Grekas <p@tchwork.com>
 * @author Alexandre Daubois <alex.daubois@gmail.com>
 *
 * @internal
 */
final class OpenSSLCaster
{
    public static function castOpensslX509(\OpenSSLCertificate $h, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castOpensslAsymmetricKey(\OpenSSLAsymmetricKey $key, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castOpensslCsr(\OpenSSLCertificateSigningRequest $csr, array $a, Stub $stub, bool $isNested): array
    {}
}
