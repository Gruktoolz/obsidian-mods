fn hook(kind: &str, stack: &ItemStack, accept: &mut bool) {
 if kind == "strangely_charming:weakness_potion" {
  *accept = matches!(stack.item.as_ref(), "minecraft:potion" | "minecraft:splash_potion" | "minecraft:lingering_potion") && matches!(stack.potion(), Some("minecraft:weakness" | "minecraft:long_weakness"));
 }
}
