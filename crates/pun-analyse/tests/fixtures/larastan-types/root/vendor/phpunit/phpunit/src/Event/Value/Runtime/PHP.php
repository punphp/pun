<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Event\Runtime;

use const PHP_EXTRA_VERSION;
use const PHP_MAJOR_VERSION;
use const PHP_MINOR_VERSION;
use const PHP_RELEASE_VERSION;
use const PHP_SAPI;
use const PHP_VERSION;
use const PHP_VERSION_ID;
use function array_merge;
use function get_loaded_extensions;
use function sort;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 */
final readonly class PHP
{
    private string $version;
    private int $versionId;
    private int $majorVersion;
    private int $minorVersion;
    private int $releaseVersion;
    private string $extraVersion;
    private string $sapi;

    /**
     * @var list<string>
     */
    private array $extensions;

    /**
     * @internal This method is not covered by the backward compatibility promise for PHPUnit
     */
    public function __construct()
    {}

    public function version(): string
    {}

    public function sapi(): string
    {}

    public function majorVersion(): int
    {}

    public function minorVersion(): int
    {}

    public function releaseVersion(): int
    {}

    public function extraVersion(): string
    {}

    public function versionId(): int
    {}

    /**
     * @return list<string>
     */
    public function extensions(): array
    {}
}
