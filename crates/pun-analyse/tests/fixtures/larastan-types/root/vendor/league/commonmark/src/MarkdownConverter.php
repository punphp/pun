<?php

declare(strict_types=1);

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark;

use League\CommonMark\Environment\EnvironmentInterface;
use League\CommonMark\Exception\CommonMarkException;
use League\CommonMark\Output\RenderedContentInterface;
use League\CommonMark\Parser\MarkdownParser;
use League\CommonMark\Parser\MarkdownParserInterface;
use League\CommonMark\Renderer\HtmlRenderer;
use League\CommonMark\Renderer\MarkdownRendererInterface;

class MarkdownConverter implements ConverterInterface, MarkdownConverterInterface
{
    /** @psalm-readonly */
    protected EnvironmentInterface $environment;

    /** @psalm-readonly */
    protected MarkdownParserInterface $markdownParser;

    /** @psalm-readonly */
    protected MarkdownRendererInterface $htmlRenderer;

    public function __construct(EnvironmentInterface $environment)
    {}

    public function getEnvironment(): EnvironmentInterface
    {}

    /**
     * Converts Markdown to HTML.
     *
     * @param string $input The Markdown to convert
     *
     * @return RenderedContentInterface Rendered HTML
     *
     * @throws CommonMarkException
     */
    public function convert(string $input): RenderedContentInterface
    {}

    /**
     * Converts Markdown to HTML.
     *
     * @deprecated since 2.2; use {@link convert()} instead
     *
     * @param string $markdown The Markdown to convert
     *
     * @return RenderedContentInterface Rendered HTML
     *
     * @throws CommonMarkException
     */
    public function convertToHtml(string $markdown): RenderedContentInterface
    {}

    /**
     * Converts CommonMark to HTML.
     *
     * @see MarkdownConverter::convert()
     *
     * @throws CommonMarkException
     */
    public function __invoke(string $markdown): RenderedContentInterface
    {}
}
