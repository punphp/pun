<?php declare(strict_types=1);
/*
 * This file is part of sebastian/object-reflector.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\ObjectReflector;

use function is_string;
use function strrpos;
use function substr;

final class ObjectReflector
{
    /**
     * Maps mangled property names to their unmangled representation, per class.
     *
     * Only mangled names (those of declared private and protected properties)
     * are cached; dynamic property names are never mangled and are used as-is.
     *
     * @var array<class-string, array<non-empty-string, string>>
     */
    private static array $names = [];

    /**
     * @return array<array-key, mixed>
     */
    public function getProperties(object $object): array
    {}
}
