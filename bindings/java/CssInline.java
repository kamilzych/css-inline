class CssInline {
    private static native String inline(String input);

    static {
        System.loadLibrary("css_inline");
    }

    public static void main(String[] args) {
        if (args.length == 0) {
            System.err.println("Please provide an input string as the first argument.");
            System.exit(1);
        }

        String input = args[0];
        String output = CssInline.inline(input);
        System.out.println(output);
    }
}
