<?php declare(strict_types=1);
/*
 * This file is part of PHPUnit.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PHPUnit\Metadata\Api;

use function array_flip;
use function array_key_exists;
use function array_unique;
use function array_values;
use function assert;
use function strtolower;
use function trim;
use PHPUnit\Framework\TestSize\TestSize;
use PHPUnit\Metadata\CoversClass;
use PHPUnit\Metadata\CoversFunction;
use PHPUnit\Metadata\Group;
use PHPUnit\Metadata\Parser\Registry;
use PHPUnit\Metadata\RequiresPhpExtension;
use PHPUnit\Metadata\UsesClass;
use PHPUnit\Metadata\UsesFunction;

/**
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for PHPUnit
 *
 * @internal This class is not covered by the backward compatibility promise for PHPUnit
 */
final class Groups
{
    /**
     * @var array<string, list<non-empty-string>>
     */
    private static array $groupCache = [];

    /**
     * The metadata that backs --covers, --uses and --requires-php-extension is
     * mapped to a group of its own so that these options can be implemented as
     * a selection by group. Everything that selects tests this way has to name
     * these groups the same way, which is why they are only named here.
     *
     * @return non-empty-string
     */
    public static function virtualGroupForCovers(string $name): string
    {}

    /**
     * @return non-empty-string
     */
    public static function virtualGroupForUses(string $name): string
    {}

    /**
     * @return non-empty-string
     */
    public static function virtualGroupForRequiredPhpExtension(string $name): string
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     *
     * @return list<non-empty-string>
     */
    public function groups(string $className, string $methodName, bool $includeVirtual = true): array
    {}

    /**
     * @param class-string     $className
     * @param non-empty-string $methodName
     */
    public function size(string $className, string $methodName): TestSize
    {}

    private static function canonicalizeName(string $name): string
    {}
}
