<?php declare(strict_types=1);
/*
 * This file is part of phpunit/php-text-template.
 *
 * (c) Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace SebastianBergmann\Template;

use function array_merge;
use function file_get_contents;
use function file_put_contents;
use function is_file;
use function is_string;
use function sprintf;
use function strtr;

final class Template
{
    /**
     * @var non-empty-string
     */
    private readonly string $template;

    /**
     * @var non-empty-string
     */
    private readonly string $openDelimiter;

    /**
     * @var non-empty-string
     */
    private readonly string $closeDelimiter;

    /**
     * @var array<string,string>
     */
    private array $values = [];

    /**
     * @param non-empty-string $templateFile
     * @param non-empty-string $openDelimiter
     * @param non-empty-string $closeDelimiter
     *
     * @throws InvalidArgumentException
     */
    public function __construct(string $templateFile, string $openDelimiter = '{', string $closeDelimiter = '}')
    {}

    /**
     * @param array<string,string> $values
     */
    public function setVar(array $values, bool $merge = true): void
    {}

    public function render(): string
    {}

    /**
     * @codeCoverageIgnore
     */
    public function renderTo(string $target): void
    {}

    /**
     * @param non-empty-string $file
     *
     * @throws InvalidArgumentException
     *
     * @return non-empty-string
     */
    private function loadTemplateFile(string $file): string
    {}
}
