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

namespace League\CommonMark\Environment;

use League\CommonMark\Delimiter\DelimiterParser;
use League\CommonMark\Delimiter\Processor\DelimiterProcessorCollection;
use League\CommonMark\Delimiter\Processor\DelimiterProcessorInterface;
use League\CommonMark\Event\DocumentParsedEvent;
use League\CommonMark\Event\ListenerData;
use League\CommonMark\Exception\AlreadyInitializedException;
use League\CommonMark\Extension\CommonMark\CommonMarkCoreExtension;
use League\CommonMark\Extension\ConfigurableExtensionInterface;
use League\CommonMark\Extension\ExtensionInterface;
use League\CommonMark\Extension\GithubFlavoredMarkdownExtension;
use League\CommonMark\Normalizer\SlugNormalizer;
use League\CommonMark\Normalizer\TextNormalizerInterface;
use League\CommonMark\Normalizer\UniqueSlugNormalizer;
use League\CommonMark\Normalizer\UniqueSlugNormalizerInterface;
use League\CommonMark\Parser\Block\BlockStartParserInterface;
use League\CommonMark\Parser\Block\SkipLinesStartingWithLettersParser;
use League\CommonMark\Parser\Inline\InlineParserInterface;
use League\CommonMark\Renderer\NodeRendererInterface;
use League\CommonMark\Util\HtmlFilter;
use League\CommonMark\Util\PrioritizedList;
use League\Config\Configuration;
use League\Config\ConfigurationAwareInterface;
use League\Config\ConfigurationInterface;
use Nette\Schema\Expect;
use Psr\EventDispatcher\EventDispatcherInterface;
use Psr\EventDispatcher\ListenerProviderInterface;
use Psr\EventDispatcher\StoppableEventInterface;

final class Environment implements EnvironmentInterface, EnvironmentBuilderInterface, ListenerProviderInterface
{
    /**
     * @var ExtensionInterface[]
     *
     * @psalm-readonly-allow-private-mutation
     */
    private array $extensions = [];

    /**
     * @var ExtensionInterface[]
     *
     * @psalm-readonly-allow-private-mutation
     */
    private array $uninitializedExtensions = [];

    /** @psalm-readonly-allow-private-mutation */
    private bool $extensionsInitialized = false;

    /**
     * @var PrioritizedList<BlockStartParserInterface>
     *
     * @psalm-readonly
     */
    private PrioritizedList $blockStartParsers;

    /**
     * @var PrioritizedList<InlineParserInterface>
     *
     * @psalm-readonly
     */
    private PrioritizedList $inlineParsers;

    /** @psalm-readonly */
    private DelimiterProcessorCollection $delimiterProcessors;

    /**
     * @var array<string, PrioritizedList<NodeRendererInterface>>
     *
     * @psalm-readonly-allow-private-mutation
     */
    private array $renderersByClass = [];

    /**
     * @var PrioritizedList<ListenerData>
     *
     * @psalm-readonly-allow-private-mutation
     */
    private PrioritizedList $listenerData;

    private ?EventDispatcherInterface $eventDispatcher = null;

    /** @psalm-readonly */
    private Configuration $config;

    private ?TextNormalizerInterface $slugNormalizer = null;

    /**
     * @param array<string, mixed> $config
     */
    public function __construct(array $config = [])
    {}

    public function getConfiguration(): ConfigurationInterface
    {}

    /**
     * @deprecated Environment::mergeConfig() is deprecated since league/commonmark v2.0 and will be removed in v3.0. Configuration should be set when instantiating the environment instead.
     *
     * @param array<string, mixed> $config
     */
    public function mergeConfig(array $config): void
    {}

    public function addBlockStartParser(BlockStartParserInterface $parser, int $priority = 0): EnvironmentBuilderInterface
    {}

    public function addInlineParser(InlineParserInterface $parser, int $priority = 0): EnvironmentBuilderInterface
    {}

    public function addDelimiterProcessor(DelimiterProcessorInterface $processor): EnvironmentBuilderInterface
    {}

    public function addRenderer(string $nodeClass, NodeRendererInterface $renderer, int $priority = 0): EnvironmentBuilderInterface
    {}

    /**
     * {@inheritDoc}
     */
    public function getBlockStartParsers(): iterable
    {}

    public function getDelimiterProcessors(): DelimiterProcessorCollection
    {}

    /**
     * {@inheritDoc}
     */
    public function getRenderersForClass(string $nodeClass): iterable
    {}

    /**
     * {@inheritDoc}
     */
    public function getExtensions(): iterable
    {}

    /**
     * Add a single extension
     *
     * @return $this
     */
    public function addExtension(ExtensionInterface $extension): EnvironmentBuilderInterface
    {}

    private function initializeExtensions(): void
    {}

    private function injectEnvironmentAndConfigurationIfNeeded(object $object): void
    {}

    /**
     * @deprecated Instantiate the environment and add the extension yourself
     *
     * @param array<string, mixed> $config
     */
    public static function createCommonMarkEnvironment(array $config = []): Environment
    {}

    /**
     * @deprecated Instantiate the environment and add the extension yourself
     *
     * @param array<string, mixed> $config
     */
    public static function createGFMEnvironment(array $config = []): Environment
    {}

    public function addEventListener(string $eventClass, callable $listener, int $priority = 0): EnvironmentBuilderInterface
    {}

    public function dispatch(object $event): object
    {}

    public function setEventDispatcher(EventDispatcherInterface $dispatcher): void
    {}

    /**
     * {@inheritDoc}
     *
     * @return iterable<callable>
     */
    public function getListenersForEvent(object $event): iterable
    { yield; }

    /**
     * @return iterable<InlineParserInterface>
     */
    public function getInlineParsers(): iterable
    {}

    public function getSlugNormalizer(): TextNormalizerInterface
    {}

    /**
     * @throws AlreadyInitializedException
     */
    private function assertUninitialized(string $message): void
    {}

    public static function createDefaultConfiguration(): Configuration
    {}
}
