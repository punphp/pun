<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\VarDumper\Caster;

use Symfony\Component\VarDumper\Cloner\Stub;

/**
 * Casts DOM related classes to array representation.
 *
 * @author Nicolas Grekas <p@tchwork.com>
 *
 * @final
 *
 * @internal
 */
class DOMCaster
{
    private const ERROR_CODES = [
        0 => 'DOM_PHP_ERR',
        \DOM_INDEX_SIZE_ERR => 'DOM_INDEX_SIZE_ERR',
        \DOMSTRING_SIZE_ERR => 'DOMSTRING_SIZE_ERR',
        \DOM_HIERARCHY_REQUEST_ERR => 'DOM_HIERARCHY_REQUEST_ERR',
        \DOM_WRONG_DOCUMENT_ERR => 'DOM_WRONG_DOCUMENT_ERR',
        \DOM_INVALID_CHARACTER_ERR => 'DOM_INVALID_CHARACTER_ERR',
        \DOM_NO_DATA_ALLOWED_ERR => 'DOM_NO_DATA_ALLOWED_ERR',
        \DOM_NO_MODIFICATION_ALLOWED_ERR => 'DOM_NO_MODIFICATION_ALLOWED_ERR',
        \DOM_NOT_FOUND_ERR => 'DOM_NOT_FOUND_ERR',
        \DOM_NOT_SUPPORTED_ERR => 'DOM_NOT_SUPPORTED_ERR',
        \DOM_INUSE_ATTRIBUTE_ERR => 'DOM_INUSE_ATTRIBUTE_ERR',
        \DOM_INVALID_STATE_ERR => 'DOM_INVALID_STATE_ERR',
        \DOM_SYNTAX_ERR => 'DOM_SYNTAX_ERR',
        \DOM_INVALID_MODIFICATION_ERR => 'DOM_INVALID_MODIFICATION_ERR',
        \DOM_NAMESPACE_ERR => 'DOM_NAMESPACE_ERR',
        \DOM_INVALID_ACCESS_ERR => 'DOM_INVALID_ACCESS_ERR',
        \DOM_VALIDATION_ERR => 'DOM_VALIDATION_ERR',
    ];

    private const NODE_TYPES = [
        \XML_ELEMENT_NODE => 'XML_ELEMENT_NODE',
        \XML_ATTRIBUTE_NODE => 'XML_ATTRIBUTE_NODE',
        \XML_TEXT_NODE => 'XML_TEXT_NODE',
        \XML_CDATA_SECTION_NODE => 'XML_CDATA_SECTION_NODE',
        \XML_ENTITY_REF_NODE => 'XML_ENTITY_REF_NODE',
        \XML_ENTITY_NODE => 'XML_ENTITY_NODE',
        \XML_PI_NODE => 'XML_PI_NODE',
        \XML_COMMENT_NODE => 'XML_COMMENT_NODE',
        \XML_DOCUMENT_NODE => 'XML_DOCUMENT_NODE',
        \XML_DOCUMENT_TYPE_NODE => 'XML_DOCUMENT_TYPE_NODE',
        \XML_DOCUMENT_FRAG_NODE => 'XML_DOCUMENT_FRAG_NODE',
        \XML_NOTATION_NODE => 'XML_NOTATION_NODE',
        \XML_HTML_DOCUMENT_NODE => 'XML_HTML_DOCUMENT_NODE',
        \XML_DTD_NODE => 'XML_DTD_NODE',
        \XML_ELEMENT_DECL_NODE => 'XML_ELEMENT_DECL_NODE',
        \XML_ATTRIBUTE_DECL_NODE => 'XML_ATTRIBUTE_DECL_NODE',
        \XML_ENTITY_DECL_NODE => 'XML_ENTITY_DECL_NODE',
        \XML_NAMESPACE_DECL_NODE => 'XML_NAMESPACE_DECL_NODE',
    ];

    public static function castException(\DOMException|\Dom\Exception $e, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castLength($dom, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castImplementation(\DOMImplementation|\Dom\Implementation $dom, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castNode(\DOMNode|\Dom\Node $dom, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castNameSpaceNode(\DOMNameSpaceNode $dom, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castDocument(\DOMDocument $dom, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}

    public static function castXMLDocument(\Dom\XMLDocument $dom, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}

    public static function castHTMLDocument(\Dom\HTMLDocument $dom, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}

    public static function castCharacterData(\DOMCharacterData|\Dom\CharacterData $dom, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castAttr(\DOMAttr|\Dom\Attr $dom, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castElement(\DOMElement|\Dom\Element $dom, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castText(\DOMText|\Dom\Text $dom, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castDocumentType(\DOMDocumentType|\Dom\DocumentType $dom, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castNotation(\DOMNotation|\Dom\Notation $dom, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castEntity(\DOMEntity|\Dom\Entity $dom, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castProcessingInstruction(\DOMProcessingInstruction|\Dom\ProcessingInstruction $dom, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castXPath(\DOMXPath|\Dom\XPath $dom, array $a, Stub $stub, bool $isNested): array
    {}

    public static function castDom($dom, array $a, Stub $stub, bool $isNested, int $filter = 0): array
    {}
}
