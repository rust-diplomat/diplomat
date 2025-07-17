// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="intro.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="user.html"><strong aria-hidden="true">2.</strong> User Guide</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="basics.html"><strong aria-hidden="true">2.1.</strong> Basics</a></li><li class="chapter-item expanded "><a href="config.html"><strong aria-hidden="true">2.2.</strong> Configuration</a></li><li class="chapter-item expanded "><a href="types.html"><strong aria-hidden="true">2.3.</strong> Types</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="opaque.html"><strong aria-hidden="true">2.3.1.</strong> Opaque types</a></li><li class="chapter-item expanded "><a href="structs.html"><strong aria-hidden="true">2.3.2.</strong> Structs and enums</a></li><li class="chapter-item expanded "><a href="option.html"><strong aria-hidden="true">2.3.3.</strong> Options</a></li><li class="chapter-item expanded "><a href="result.html"><strong aria-hidden="true">2.3.4.</strong> Results</a></li><li class="chapter-item expanded "><a href="writeable.html"><strong aria-hidden="true">2.3.5.</strong> Returning Strings: Writeable</a></li></ol></li><li class="chapter-item expanded "><a href="docs.html"><strong aria-hidden="true">2.4.</strong> Documentation</a></li><li class="chapter-item expanded "><a href="lifetimes.html"><strong aria-hidden="true">2.5.</strong> Lifetimes</a></li><li class="chapter-item expanded "><a href="abi.html"><strong aria-hidden="true">2.6.</strong> ABI naming/renaming</a></li><li class="chapter-item expanded "><a href="attrs.html"><strong aria-hidden="true">2.7.</strong> Customizing via attributes</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="attrs/disable.html"><strong aria-hidden="true">2.7.1.</strong> Disabling APIs</a></li><li class="chapter-item expanded "><a href="attrs/rename.html"><strong aria-hidden="true">2.7.2.</strong> Renaming APIs</a></li><li class="chapter-item expanded "><a href="attrs/namespace.html"><strong aria-hidden="true">2.7.3.</strong> Namespacing</a></li><li class="chapter-item expanded "><a href="attrs/constructors.html"><strong aria-hidden="true">2.7.4.</strong> Constructors</a></li><li class="chapter-item expanded "><a href="attrs/iterators.html"><strong aria-hidden="true">2.7.5.</strong> Iterators and iterables</a></li><li class="chapter-item expanded "><a href="attrs/accessors.html"><strong aria-hidden="true">2.7.6.</strong> Getters and setters</a></li><li class="chapter-item expanded "><a href="attrs/indexing.html"><strong aria-hidden="true">2.7.7.</strong> Indexing</a></li><li class="chapter-item expanded "><a href="attrs/arithmetic.html"><strong aria-hidden="true">2.7.8.</strong> Arithmetic</a></li><li class="chapter-item expanded "><a href="attrs/comparators.html"><strong aria-hidden="true">2.7.9.</strong> Comparators</a></li><li class="chapter-item expanded "><a href="attrs/stringifiers.html"><strong aria-hidden="true">2.7.10.</strong> Stringifiers</a></li><li class="chapter-item expanded "><a href="attrs/slices.html"><strong aria-hidden="true">2.7.11.</strong> Slices</a></li></ol></li><li class="chapter-item expanded "><a href="safety.html"><strong aria-hidden="true">2.8.</strong> Notes on Diplomat and safety</a></li></ol></li><li class="chapter-item expanded "><a href="developer.html"><strong aria-hidden="true">3.</strong> Backend developer guide</a></li><li class="chapter-item expanded "><a href="demo_gen/intro.html"><strong aria-hidden="true">4.</strong> demo_gen</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="demo_gen/quickstart.html"><strong aria-hidden="true">4.1.</strong> Quickstart</a></li><li class="chapter-item expanded "><a href="demo_gen/attributes.html"><strong aria-hidden="true">4.2.</strong> Attributes</a></li><li class="chapter-item expanded "><a href="demo_gen/markup.html"><strong aria-hidden="true">4.3.</strong> Configuring Markup</a></li><li class="chapter-item expanded "><a href="demo_gen/renderer.html"><strong aria-hidden="true">4.4.</strong> Configuring the Default Renderer</a></li><li class="chapter-item expanded "><a href="demo_gen/custom_renderer.html"><strong aria-hidden="true">4.5.</strong> Making Your Own Renderer</a></li><li class="chapter-item expanded "><a href="demo_gen/custom_functions.html"><strong aria-hidden="true">4.6.</strong> Making Custom Functions</a></li></ol></li><li class="chapter-item expanded "><a href="backends/intro.html"><strong aria-hidden="true">5.</strong> Backend specific documentation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="backends/kotlin.html"><strong aria-hidden="true">5.1.</strong> Kotlin</a></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
