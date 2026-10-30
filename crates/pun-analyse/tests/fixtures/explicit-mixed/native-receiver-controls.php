<?php

final class NativeReceiverControls
{
    public int $integer;
    public mixed $prop;
    /** @var mixed */
    public $documentedOnly;

    /** @return mixed */
    public function documentedReturn() { return 1; }
    /** @return int */
    public function documentedInt() { return 1; }
    public function scalarReturn(): int { return 1; }

    public function check(NativeReceiverControls $ordinary): void
    {
        $this->integer; // mixed: native-integer-property
        $ordinary->documentedOnly; // mixed: ordinary-documented-untyped-property
        $this->documentedReturn(); // mixed: this-documented-return
        $ordinary->documentedReturn(); // mixed: ordinary-documented-return
        $this->documentedInt(); // mixed: documented-integer-without-native-return
        $this->scalarReturn(); // mixed: native-integer-return
        (new NativeReceiverControls())->prop; // mixed: constructed-expression-receiver
    }
}
