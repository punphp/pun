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

use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\Uid\TimeBasedUidInterface;
use Symfony\Component\Uid\Ulid;
use Symfony\Component\Uid\Uuid;
use Symfony\Component\VarDumper\Cloner\Stub;
use Symfony\Component\VarExporter\Internal\LazyObjectState;

/**
 * @final
 *
 * @internal
 */
class SymfonyCaster
{
    private const REQUEST_GETTERS = [
        'pathInfo' => 'getPathInfo',
        'requestUri' => 'getRequestUri',
        'baseUrl' => 'getBaseUrl',
        'basePath' => 'getBasePath',
        'method' => 'getMethod',
        'format' => 'getRequestFormat',
    ];

    public static function castRequest(Request $request, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castHttpClient($client, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castHttpClientResponse($response, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castLazyObjectState($state, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castUuid(Uuid $uuid, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castUlid(Ulid $ulid, array $a, Stub $stub, bool $isNested): array
    {}
}
