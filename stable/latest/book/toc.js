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
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><a href="introduction.html">Introduction</a></li><li class="chapter-item expanded "><a href="installation.html"><strong aria-hidden="true">1.</strong> Installation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="installation_linux.html"><strong aria-hidden="true">1.1.</strong> Linux</a></li><li class="chapter-item expanded "><a href="installation_macos.html"><strong aria-hidden="true">1.2.</strong> macOS</a></li><li class="chapter-item expanded "><a href="installation_windows.html"><strong aria-hidden="true">1.3.</strong> Windows</a></li></ol></li><li class="chapter-item expanded "><a href="project_setup.html"><strong aria-hidden="true">2.</strong> Project Setup</a></li><li class="chapter-item expanded "><a href="hello_world.html"><strong aria-hidden="true">3.</strong> Hello World!</a></li><li class="chapter-item expanded "><a href="widgets.html"><strong aria-hidden="true">4.</strong> Widgets</a></li><li class="chapter-item expanded "><a href="g_object_concepts.html"><strong aria-hidden="true">5.</strong> GObject Concepts</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="g_object_memory_management.html"><strong aria-hidden="true">5.1.</strong> Memory Management</a></li><li class="chapter-item expanded "><a href="g_object_subclassing.html"><strong aria-hidden="true">5.2.</strong> Subclassing</a></li><li class="chapter-item expanded "><a href="g_object_values.html"><strong aria-hidden="true">5.3.</strong> Generic Values</a></li><li class="chapter-item expanded "><a href="g_object_properties.html"><strong aria-hidden="true">5.4.</strong> Properties</a></li><li class="chapter-item expanded "><a href="g_object_signals.html"><strong aria-hidden="true">5.5.</strong> Signals</a></li></ol></li><li class="chapter-item expanded "><a href="main_event_loop.html"><strong aria-hidden="true">6.</strong> The Main Event Loop</a></li><li class="chapter-item expanded "><a href="settings.html"><strong aria-hidden="true">7.</strong> Settings</a></li><li class="chapter-item expanded "><a href="saving_window_state.html"><strong aria-hidden="true">8.</strong> Saving Window State</a></li><li class="chapter-item expanded "><a href="list_widgets.html"><strong aria-hidden="true">9.</strong> List Widgets</a></li><li class="chapter-item expanded "><a href="composite_templates.html"><strong aria-hidden="true">10.</strong> Composite Templates</a></li><li class="chapter-item expanded "><a href="todo_1.html"><strong aria-hidden="true">11.</strong> Building a Simple To-Do App</a></li><li class="chapter-item expanded "><a href="actions.html"><strong aria-hidden="true">12.</strong> Actions</a></li><li class="chapter-item expanded "><a href="todo_2.html"><strong aria-hidden="true">13.</strong> Manipulating State of To-Do App</a></li><li class="chapter-item expanded "><a href="css.html"><strong aria-hidden="true">14.</strong> CSS</a></li><li class="chapter-item expanded "><a href="libadwaita.html"><strong aria-hidden="true">15.</strong> Libadwaita</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="todo_3.html"><strong aria-hidden="true">15.1.</strong> Let To-Do App use Libadwaita</a></li><li class="chapter-item expanded "><a href="todo_4.html"><strong aria-hidden="true">15.2.</strong> Adding Collections</a></li></ol></li></ol>';
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
