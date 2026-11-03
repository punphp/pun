<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-code-coverage.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\CodeCoverage\Report\Html;

use function is_file;
use SebastianBergmann\CodeCoverage\InvalidArgumentException;

/**
 * @immutable
 *
 * @no-named-arguments Parameter names are not covered by the backward compatibility promise for phpunit/php-code-coverage
 */
final readonly class CustomCssFile
{
    private string $path;

    public static function default(): self
    {}

    /**
     * @throws InvalidArgumentException
     */
    public static function from(string $path): self
    {}

    private function __construct(string $path)
    {}

    public function path(): string
    {}
}
