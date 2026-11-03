<?php

/*
 * This file is part of the Symfony package.
 *
 * (c) Fabien Potencier <fabien@symfony.com>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */

namespace Symfony\Component\Console\Descriptor;

use Symfony\Component\Console\Application;
use Symfony\Component\Console\Command\Command;
use Symfony\Component\Console\Input\InputArgument;
use Symfony\Component\Console\Input\InputDefinition;
use Symfony\Component\Console\Input\InputOption;

/**
 * XML descriptor.
 *
 * @author Jean-François Simon <contact@jfsimon.fr>
 *
 * @internal
 */
class XmlDescriptor extends Descriptor
{
    public function getInputDefinitionDocument(InputDefinition $definition): \DOMDocument
    {}

    public function getCommandDocument(Command $command, bool $short = false): \DOMDocument
    {}

    public function getApplicationDocument(Application $application, ?string $namespace = null, bool $short = false): \DOMDocument
    {}

    protected function describeInputArgument(InputArgument $argument, array $options = []): void
    {}

    protected function describeInputOption(InputOption $option, array $options = []): void
    {}

    protected function describeInputDefinition(InputDefinition $definition, array $options = []): void
    {}

    protected function describeCommand(Command $command, array $options = []): void
    {}

    protected function describeApplication(Application $application, array $options = []): void
    {}

    /**
     * Appends document children to parent node.
     */
    private function appendDocument(\DOMNode $parentNode, \DOMNode $importedParent): void
    {}

    /**
     * Writes DOM document.
     */
    private function writeDocument(\DOMDocument $dom): void
    {}

    private function getInputArgumentDocument(InputArgument $argument): \DOMDocument
    {}

    private function getInputOptionDocument(InputOption $option): \DOMDocument
    {}
}
