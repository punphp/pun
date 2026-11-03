<?php

declare(strict_types=1);

/*
 * This file is part of the league/commonmark package.
 *
 * (c) Colin O'Dell <colinodell@gmail.com>
 *
 * Original code based on the CommonMark JS reference parser (https://bitly.com/commonmark-js)
 *  - (c) John MacFarlane
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace League\CommonMark\Renderer;

use League\CommonMark\Environment\EnvironmentInterface;
use League\CommonMark\Event\DocumentPreRenderEvent;
use League\CommonMark\Event\DocumentRenderedEvent;
use League\CommonMark\Node\Block\AbstractBlock;
use League\CommonMark\Node\Block\Document;
use League\CommonMark\Node\Node;
use League\CommonMark\Output\RenderedContent;
use League\CommonMark\Output\RenderedContentInterface;

final class HtmlRenderer implements DocumentRendererInterface, ChildNodeRendererInterface
{
    /** @psalm-readonly */
    private EnvironmentInterface $environment;

    public function __construct(EnvironmentInterface $environment)
    {}

    public function renderDocument(Document $document): RenderedContentInterface
    {}

    /**
     * {@inheritDoc}
     */
    public function renderNodes(iterable $nodes): string
    {}

    /**
     * @return \Stringable|string
     *
     * @throws NoMatchingRendererException
     */
    private function renderNode(Node $node)
    {}

    public function getBlockSeparator(): string
    {}

    public function getInnerSeparator(): string
    {}
}
