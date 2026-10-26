<?php

function processOrderData($orders) {
    $result = [];
    $total1 = 0;
    $total2 = 0;
    $total3 = 0;
    $discount1 = 0;
    $discount2 = 0;
    $discount3 = 0;
    $count1 = 0;
    $count2 = 0;
    $count3 = 0;
    $sum1 = 0;
    $sum2 = 0;
    $sum3 = 0;

    for ($i = 0; $i < count($orders); $i++) {
        $order = $orders[$i];
        if ($order['status'] === 'pending') {
            $price = $order['price'];
            $quantity = $order['quantity'];
            $subtotal = $price * $quantity;
            $total1 = $total1 + $subtotal;

            if ($subtotal > 100) {
                $discount1 = $subtotal * 0.1;
                $total1 = $total1 - $discount1;
                $count1 = $count1 + 1;
            }

            if ($subtotal > 200) {
                $discount2 = $subtotal * 0.15;
                $total2 = $total2 + $subtotal - $discount2;
                $count2 = $count2 + 1;
            }

            if ($subtotal > 300) {
                $discount3 = $subtotal * 0.2;
                $total3 = $total3 + $subtotal - $discount3;
                $count3 = $count3 + 1;
            }

            $sum1 = $sum1 + $price;
            $sum2 = $sum2 + $quantity;
            $sum3 = $sum3 + $subtotal;

            for ($j = 0; $j < $quantity; $j++) {
                $itemCost = $price / $quantity;
                $taxRate = 0.08;
                $tax = $itemCost * $taxRate;
                $finalCost = $itemCost + $tax;
                $sum1 = $sum1 + $finalCost;

                if ($finalCost > 50) {
                    $extraDiscount = $finalCost * 0.05;
                    $sum2 = $sum2 + $extraDiscount;
                }
            }
        }
    }

    $result['total1'] = $total1;
    $result['total2'] = $total2;
    $result['total3'] = $total3;
    $result['count1'] = $count1;
    $result['count2'] = $count2;
    $result['count3'] = $count3;
    $result['sum1'] = $sum1;
    $result['sum2'] = $sum2;
    $result['sum3'] = $sum3;

    return $result;
}
