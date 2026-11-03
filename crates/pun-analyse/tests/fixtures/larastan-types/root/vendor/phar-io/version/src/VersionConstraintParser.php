<?php declare(strict_types = 1);
/*
 * This file is part of PharIo\Version.
 *
 * (c) Arne Blankerts <arne@blankerts.de>, Sebastian Heuer <sebastian@phpeople.de>, Sebastian Bergmann <sebastian@phpunit.de>
 *
 * For the full copyright and license information, please view the LICENSE
 * file that was distributed with this source code.
 */
namespace PharIo\Version;

class VersionConstraintParser {
    /**
     * @throws UnsupportedVersionConstraintException
     */
    public function parse(string $value): VersionConstraint {}

    private function handleOrGroup(string $value): OrVersionConstraintGroup {}

    private function handleTildeOperator(string $value): AndVersionConstraintGroup {}

    private function handleCaretOperator(string $value): AndVersionConstraintGroup {}
}
