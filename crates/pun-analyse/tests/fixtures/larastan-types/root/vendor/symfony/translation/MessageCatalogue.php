<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Translation;

use Symfony\Component\Config\Resource\ResourceInterface;
use Symfony\Component\Translation\Exception\LogicException;

/**
 * @author Fabien Potencier <fabien@symfony.com>
 */
class MessageCatalogue implements MessageCatalogueInterface, MetadataAwareInterface, CatalogueMetadataAwareInterface
{
    private array $metadata = [];
    private array $catalogueMetadata = [];
    private array $resources = [];
    private ?MessageCatalogueInterface $fallbackCatalogue = null;
    private ?self $parent = null;

    /**
     * @param array $messages An array of messages classified by domain
     */
    public function __construct(
        private string $locale,
        private array $messages = [],
    ) {}

    public function getLocale(): string
    {}

    public function getDomains(): array
    {}

    public function all(?string $domain = null): array
    {}

    public function set(string $id, string $translation, string $domain = 'messages'): void
    {}

    public function has(string $id, string $domain = 'messages'): bool
    {}

    public function defines(string $id, string $domain = 'messages'): bool
    {}

    public function get(string $id, string $domain = 'messages'): string
    {}

    public function replace(array $messages, string $domain = 'messages'): void
    {}

    public function add(array $messages, string $domain = 'messages'): void
    {}

    public function addCatalogue(MessageCatalogueInterface $catalogue): void
    {}

    public function addFallbackCatalogue(MessageCatalogueInterface $catalogue): void
    {}

    public function getFallbackCatalogue(): ?MessageCatalogueInterface
    {}

    public function getResources(): array
    {}

    public function addResource(ResourceInterface $resource): void
    {}

    public function getMetadata(string $key = '', string $domain = 'messages'): mixed
    {}

    public function setMetadata(string $key, mixed $value, string $domain = 'messages'): void
    {}

    public function deleteMetadata(string $key = '', string $domain = 'messages'): void
    {}

    public function getCatalogueMetadata(string $key = '', string $domain = 'messages'): mixed
    {}

    public function setCatalogueMetadata(string $key, mixed $value, string $domain = 'messages'): void
    {}

    public function deleteCatalogueMetadata(string $key = '', string $domain = 'messages'): void
    {}

    /**
     * Adds current values with the new values.
     *
     * @param array $values Values to add
     */
    private function addMetadata(array $values): void
    {}

    private function addCatalogueMetadata(array $values): void
    {}
}
