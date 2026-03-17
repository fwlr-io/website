use crate::codeblock;
use crate::termblock;
use crate::ux::*;
use leptos::prelude::*;

#[component]
pub fn ModernTerminal() -> impl IntoView {
    view! {
        <br />

        <p>
            r##"
                Many basic shell commands have superior counterparts.
                For some it's improved performance; for others, it's improved ergonomics;
                for a few, it's the addition of entirely new capabilites.
            "##
        </p>
        <p>
            r##"
                Consider
            "##<C c="cat" />":"
        </p>
        <termblock::Cat />
        <p>
            r##"
                Upgrade to
            "##<a href="https://github.com/sharkdp/bat">"bat"</a>r##"
                for git awareness and syntax highlighting:
            "##
        </p>
        <codeblock::InstallBat />
        <termblock::Bat />

        <Break />

        <p>
            r##"
                Or take
            "##<C c="ls" />":"
        </p>
        <termblock::LsShort />
        <termblock::LsLong />
        <p>
            r##"
                Upgrade to
            "##<a href="https://eza.rocks">"eza"</a>r##"
                for tree display, respecting local git ignores,
                filenames hyperlinked to open in editor on click,
                even icons if you desire it.
            "##
        </p>
        <codeblock::EzaConfig />
        <termblock::EzaShort />
        <termblock::EzaLong />

        <Break />

        <p>
            r##"
                Jump to regularly-used directories with
            "## <a href="https://github.com/agkozak/zsh-z">"zsh-z"</a>":"
        </p>
        <termblock::Z />
        <p>
            r##"
                This is a common idea; there are alternatives like
            "##<a href="https://github.com/wting/autojump">"autojump"</a>", "
            r##"
                and many more with various unsearchable names like "fj".
                A search string like "fuzzy frecency jump" might find something suitable.
            "##
        </p>

        <Break />

        <p>
            r##"
                Most of my attempts to use
            "##<C c="find" />r##"
                look something like this:
            "##
        </p>
        <termblock::Find />
        <p>
            r##"
                Upgrade to
            "##<a href="https://github.com/sharkdp/fd">fd</a>r##"
                for faster searches with the syntax you'd expect.
            "##
        </p>
        <codeblock::InstallFd />
        <termblock::Fd />

        <Break />

        <p>
            r##"
                A similarly finicky utility is
            "##<C c="grep" />":"
        </p>
        <termblock::Grep />

        <p>
            r##"
                Upgrade to
            "##<a href="https://github.com/BurntSushi/ripgrep">rg</a>r##"
                for greatly improved performance, better defaults, and saner syntax.
            "##
        </p>
        <codeblock::InstallRg />
        <termblock::Rg />
        <p>
            r##"
                It's particularly advantageous for regex users. Compare the original...
            "##
        </p>
        <termblock::GrepRegex />
        <p>
            r##"
                ...to the upgrade:
            "##
        </p>
        <termblock::RgRegex />
        <p>
            r##"
                Convenient in-line replace:
            "##
        </p>
        <termblock::RgReplace />
        <p>
            r##"
                It automatically detects PCRE regexes, and only engages the slower PCRE-supporting engine when needed:
            "##
        </p>
        <termblock::RgPcre />

        <Break />

        <p>
            r##"
                The standard
            "##<C c="git diff" />r##"
                pager is serviceable, but it's not exactly comfortable:
            "##
        </p>
        <termblock::DiffLess />
        <p>
            r##"
                It's the current year - you can be comfortable without being reduced
                to poking at Git through an IDE (or, heaven forfend, a
            "##<a href="https://github.com">"React app"</a>r##"
                on
            "##<a href="https://blog.codinghorror.com/content/images/2019/02/there-is-no-cloud.png">r##"
                someone else's computer
            "##</a>r##"
                ). You can use
            "##<a href="https://dandavison.github.io/delta/">"delta"</a>r##"
                instead!
            "##
        </p>
        <codeblock::InstallDelta />
        <codeblock::DeltaConfig />
        <termblock::DiffDelta />

        <Break />

        <p>
            r##"
                Going beyond "drop-in" replacements,
            "##<a href="https://github.com/junegunn/fzf">fzf</a>r##"
                can transform turn a command line into your own personal TUI.
            "##
        </p>
        <codeblock::InstallFzf />

        <p>
            r##"
                Option+C to fuzzy-search a directory to "##<C c="cd" />r##" into:
            "##
        </p>
        <termblock::FzfCd />

        <p>
            r##"
                Control+T to fuzzy-search and preview files, Tab to multi select, then
               Enter to append them to your current terminal line.
            "##
        </p>
        <termblock::FzfFs />

        <p>
            r##"
                It supports many of the previous tools...
            "##
        </p>
        <termblock::FzfGitDelta tiny=true />
        <termblock::FzfGitBat tiny=true />
        <p>
            r##"
                ...although the configuration involved is extensive.
                This particular rabbithole is rather unlikely
                to pay off in time saved.
            "##
        </p>
        <codeblock::FzfConfig />

        <Break />

        <p>
            r##"
                How about an
            "##<a href="https://github.com/reegnz/jq-zsh-plugin">"interactive live repl"</a>r##"
                for "##<C c="jq" />r##"?
            "##
        </p>
        <termblock::FzfJq tiny=true />

        <br />
    }
}
