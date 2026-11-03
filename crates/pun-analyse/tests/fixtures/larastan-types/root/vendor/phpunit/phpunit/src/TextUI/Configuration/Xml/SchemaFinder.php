<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\TextUI\XmlConfiguration;

use function assert;
use function defined;
use function is_file;
use function is_string;
use function rsort;
use function sprintf;
use DirectoryIterator;
use PHPUnit\Runner\Version;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final readonly class SchemaFinder
{
    /**
     * @return non-empty-list<non-empty-string>
     */
    public function available(): array
    {}

    /**
     * @throws CannotFindSchemaException
     */
    public function find(string $version): string
    {}

    private function path(): string
    {}
}
