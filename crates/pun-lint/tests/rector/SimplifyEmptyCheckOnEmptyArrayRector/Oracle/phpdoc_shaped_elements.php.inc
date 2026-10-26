<?php

final class Frame
{
}

final class PhpDocShapedElements
{
    /**
     * @return list<Frame>
     */
    private function frames()
    {
        return [];
    }

    public function groups(): array
    {
        $groups = [];

        foreach ($this->frames() as $frame) {
            if (empty($groups)) {
                $groups[] = ['frames' => [$frame]];
            }
        }

        return $groups;
    }
}
