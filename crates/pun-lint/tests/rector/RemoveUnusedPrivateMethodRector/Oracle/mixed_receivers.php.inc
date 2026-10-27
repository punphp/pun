<?php

namespace Pun\Oracle\MixedReceivers;

final class OtherHolder
{
    public function other()
    {
    }
}

final class MixedReceivers
{
    public function run(mixed $m)
    {
        $this->nativeMixed()->viaNativeMixed();
        $this->docMixed()->viaDocMixed();
        $m->viaExplicitVariable();
        $this->documentedOther()->viaDocumentedOther();
        $this?->untyped()->viaNullsafeRoot();
        $this->untyped()?->viaNullsafeLink();
    }

    private function nativeMixed(): mixed
    {
        return $this;
    }

    /**
     * @return mixed
     */
    private function docMixed()
    {
        return $this;
    }

    /**
     * @return OtherHolder
     */
    private function documentedOther()
    {
        return $this;
    }

    private function untyped()
    {
        return $this;
    }

    private function viaNativeMixed()
    {
    }

    private function viaDocMixed()
    {
    }

    private function viaExplicitVariable()
    {
    }

    private function viaDocumentedOther()
    {
    }

    private function viaNullsafeRoot()
    {
    }

    private function viaNullsafeLink()
    {
    }
}
-----
<?php

namespace Pun\Oracle\MixedReceivers;

final class OtherHolder
{
    public function other()
    {
    }
}

final class MixedReceivers
{
    public function run(mixed $m)
    {
        $this->nativeMixed()->viaNativeMixed();
        $this->docMixed()->viaDocMixed();
        $m->viaExplicitVariable();
        $this->documentedOther()->viaDocumentedOther();
        $this?->untyped()->viaNullsafeRoot();
        $this->untyped()?->viaNullsafeLink();
    }

    private function nativeMixed(): mixed
    {
        return $this;
    }

    /**
     * @return mixed
     */
    private function docMixed()
    {
        return $this;
    }

    /**
     * @return OtherHolder
     */
    private function documentedOther()
    {
        return $this;
    }

    private function untyped()
    {
        return $this;
    }

    private function viaNativeMixed()
    {
    }

    private function viaDocMixed()
    {
    }

    private function viaNullsafeLink()
    {
    }
}
