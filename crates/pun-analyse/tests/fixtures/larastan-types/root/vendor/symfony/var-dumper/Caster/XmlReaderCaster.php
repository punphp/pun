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
 * Casts XmlReader class to array representation.
 *
 * @author Baptiste Clavié <clavie.b@gmail.com>
 *
 * @final
 *
 * @internal
 */
class XmlReaderCaster
{
    private const NODE_TYPES = [
        \XMLReader::NONE => 'NONE',
        \XMLReader::ELEMENT => 'ELEMENT',
        \XMLReader::ATTRIBUTE => 'ATTRIBUTE',
        \XMLReader::TEXT => 'TEXT',
        \XMLReader::CDATA => 'CDATA',
        \XMLReader::ENTITY_REF => 'ENTITY_REF',
        \XMLReader::ENTITY => 'ENTITY',
        \XMLReader::PI => 'PI (Processing Instruction)',
        \XMLReader::COMMENT => 'COMMENT',
        \XMLReader::DOC => 'DOC',
        \XMLReader::DOC_TYPE => 'DOC_TYPE',
        \XMLReader::DOC_FRAGMENT => 'DOC_FRAGMENT',
        \XMLReader::NOTATION => 'NOTATION',
        \XMLReader::WHITESPACE => 'WHITESPACE',
        \XMLReader::SIGNIFICANT_WHITESPACE => 'SIGNIFICANT_WHITESPACE',
        \XMLReader::END_ELEMENT => 'END_ELEMENT',
        \XMLReader::END_ENTITY => 'END_ENTITY',
        \XMLReader::XML_DECLARATION => 'XML_DECLARATION',
    ];

    public static function castXmlReader(\XMLReader $reader, array $a, Stub $stub, bool $isNested): array
    {}
}
