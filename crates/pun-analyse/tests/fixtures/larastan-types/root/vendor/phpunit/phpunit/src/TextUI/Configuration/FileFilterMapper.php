<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\Configuration;

use function realpath;
use SebastianBergmann\FileFilter\Builder as FilterBuilder;
use SebastianBergmann\FileFilter\Filter as FileFilter;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class FileFilterMapper
{
    public function map(Source $source): FileFilter
    {}

    /**
     * @return list<array{path: non-empty-string, prefix: string, suffix: string}>
     */
    private function directories(FilterDirectoryCollection $directories): array
    {}

    /**
     * @return list<non-empty-string>
     */
    private function files(FilterFileCollection $files): array
    {}
}
