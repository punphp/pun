<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Runner\Extension;

use function count;
use function explode;
use function extension_loaded;
use function implode;
use function is_file;
use function sprintf;
use function str_contains;
use PharIo\Manifest\ApplicationName;
use PharIo\Manifest\Exception as ManifestException;
use PharIo\Manifest\ManifestLoader;
use PharIo\Version\Version as PharIoVersion;
use PHPUnit\Event\Emitter;
use PHPUnit\Runner\Version;
use SebastianBergmann\FileIterator\Facade as FileIteratorFacade;
use Throwable;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class PharLoader
{
    private Emitter $emitter;

    public function __construct(Emitter $emitter)
    {}

    /**
     * @param non-empty-string $directory
     *
     * @return list<string>
     */
    public function loadPharExtensionsInDirectory(string $directory): array
    {}

    private function phpunitVersion(): string
    {}
}
