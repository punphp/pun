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

namespace League\CommonMark\Extension\DisallowedRawHtml;

use League\CommonMark\Node\Node;
use League\CommonMark\Renderer\ChildNodeRendererInterface;
use League\CommonMark\Renderer\NodeRendererInterface;
use League\Config\ConfigurationAwareInterface;
use League\Config\ConfigurationInterface;

final class DisallowedRawHtmlRenderer implements NodeRendererInterface, ConfigurationAwareInterface
{
    /** @psalm-readonly */
    private NodeRendererInterface $innerRenderer;

    /** @psalm-readonly-allow-private-mutation */
    private ConfigurationInterface $config;

    public function __construct(NodeRendererInterface $innerRenderer)
    {}

    public function render(Node $node, ChildNodeRendererInterface $childRenderer): ?string
    {}

    public function setConfiguration(ConfigurationInterface $configuration): void
    {}
}
