const SCRIPT: &str = r#"
<div id="myModal" class="modal">
    <div class="modal-content" id="modal-content">
        <div id="container">
            <div class="word-box">
                <h1 id="original"></h1>
                <button id="audio-btn">Play</button>
            </div>
            <div id="translation"></div>
        </div>
        <div id="spinner"></div>
    </div>
</div>

<style>
    .word-box {
        display: flex;
        gap: 10px;
        justifyContent: 'center';
    }

    /* CSS for modal */
    .modal {
        display: none; /* Hidden by default */
        position: fixed; /* Stay in place */
        z-index: 1; /* Sit on top */
        left: 0;
        top: 0;
        width: 100%; /* Full width */
        height: 100%; /* Full height */
        overflow: auto; /* Enable scroll if needed */
        background-color: rgb(0,0,0); /* Fallback color */
        background-color: rgba(0,0,0,0.4); /* Black w/ opacity */
    }

    .modal-content {
        background-color: #fefefe;
        margin: 15% auto; /* 15% from the top and centered */
        padding: 20px;
        border: 1px solid #888;
        width: 80%; /* Could be more or less, depending on screen size */
    }

    .close {
        color: #aaa;
        float: right;
        font-size: 28px;
        font-weight: bold;
    }

    .close:hover,
    .close:focus {
        color: black;
        text-decoration: none;
        cursor: pointer;
    }
</style>
<script type="text/javascript">
    const CircularLoader = {
        loader: null,
        canvas: null,
        ctx: null,
        size: 50,
        lineWidth: 5,
        rotationSpeed: 5,
        color: '#000000',
        rotation: 0,

        init(container, options = {}) {
            this.size = options.size || this.size;
            this.lineWidth = options.lineWidth || this.lineWidth;
            this.rotationSpeed = options.rotationSpeed || this.rotationSpeed;
            this.color = options.color || this.color;

            this.canvas = document.createElement('canvas');
            this.canvas.width = this.size;
            this.canvas.height = this.size;
            this.ctx = this.canvas.getContext('2d');

            this.loader = document.createElement('div');
            this.loader.style.width = `${this.size}px`;
            this.loader.style.height = `${this.size}px`;
            this.loader.style.position = 'relative';
            this.loader.style.display = 'inline-block';
            this.loader.appendChild(this.canvas);

            if (container) {
                container.appendChild(this.loader);
            }

            this.draw();
            this.start();

            return this.loader;
        },

        draw() {
            const centerX = this.size / 2;
            const centerY = this.size / 2;
            const radius = (this.size - this.lineWidth) / 2;

            this.ctx.clearRect(0, 0, this.size, this.size);
            this.ctx.beginPath();
            this.ctx.arc(centerX, centerY, radius, 0, Math.PI * 1.5);
            this.ctx.strokeStyle = this.color;
            this.ctx.lineWidth = this.lineWidth;
            this.ctx.lineCap = 'round';
            this.ctx.stroke();
        },

        rotate() {
            this.rotation += this.rotationSpeed;
            this.canvas.style.transform = `rotate(${this.rotation}deg)`;
            requestAnimationFrame(() => this.rotate());
        },

        start() {
            this.rotate();
        },

        stop() {
            cancelAnimationFrame(this.rotate);
        },

        show() {
            this.loader.style.display = 'inline-block';
        },

        hide() {
            this.loader.style.display = 'none';
        }
    };

    async function query(text) {
        const response = await fetch("http://127.0.0.1:8000/translate", {
            method: 'POST',
            body: JSON.stringify({text: text}),
            headers: {
                "Content-Type": "application/json",
            }
        });
        const jsonResponse = await response.json();
        return jsonResponse;
    }

    async function generateSpeech(text, language) {
        const response = await fetch("http://localhost:8000/synth", {
            method: 'POST',
            body: JSON.stringify({
                text: text
            }),
            headers: {
                "Content-Type": "application/json",
            }
        })
        return await response.blob();
    }

    function stripPuncs(text) {
        return text
            .replace('.', '')
            .replace(',', '')
            .replace('?', '')
            .replace('!', '')
            .replace('¿', '');
    }

    const modal = document.getElementById('myModal');
    const modalContent = document.getElementById('modal-content');
    const original = document.getElementById('original');
    const translation = document.getElementById('translation');
    const audioButton = document.getElementById('audio-btn');
    const container = document.getElementById('container');
    const spinner = document.getElementById('spinner');

    const loader = Object.create(CircularLoader);
    loader.init(spinner, {
        size: 60,
        lineWidth: 6,
        rotationSpeed: 8,
        color: '#3498db'
    });

    window.onclick = function(event) {
        if (event.target == modal) {
            modal.style.display = 'none';
        }
    }

    window.currentAudioBlob = undefined;
    audioButton.onclick = async function () {
        if (window.currentAudioBlob) {
            const audioUrl = URL.createObjectURL(window.currentAudioBlob);
            const audio = new Audio(audioUrl);
            await audio.play();
        }
    }

    window.addEventListener('DOMContentLoaded', function () {
        window.translate = function(element) {
            container.style.display = 'none';
            loader.start();
            loader.show();
            modal.style.display = 'block';
            window.currentAudioBlob = undefined;
            const text = stripPuncs(element.innerText);
            query(text).then(jsonResponse => {
                original.innerText = text;
                translation.innerText = jsonResponse.translation_text;
                window.currentAudioBlob = 
                generateSpeech(text, "fr").then(audioBlob => {
                    window.currentAudioBlob = audioBlob;
                    loader.stop();
                    loader.hide();
                    container.style.display = 'block';
                });
            });

        }
    });

</script>"#;

pub fn wrap_words_in_paragraphs(html: &str) -> String {
    let mut html_string = String::from(html);
    
    if let Some(body_index) = html_string.rfind("</body>") {
        html_string.insert_str(body_index, SCRIPT);
    } else {
        // If there's no closing body tag, we could just append it at the end
        html_string.push_str(SCRIPT);
    }

    let mut current_index = 0;
    
    while let Some(index) = find_substring_from_index(&html_string, "<p", current_index) {
        if current_index > html_string.len() {
            panic!("Current index beyond bounds of html string.");
        }
        if let Some(closing_p_index) = find_substring_from_index(&html_string, "</p>", index) {
            let opening_tag_end = html_string[index..].find('>').map(|i| i + index + 1)
                .unwrap_or_else(|| panic!("Malformed html. Could not find closing '>' for opening p tag."));
            let paragraph_text = &html_string[opening_tag_end..closing_p_index];
            let mut modified_p_text = String::new();
            let mut inside_word = false;
            let mut inside_span = false;
            let mut inside_tag = false;
            let mut current_span = String::new();
            let mut span_buffer = String::new();
            let mut tag_buffer = String::new();
            
            let mut chars = paragraph_text.chars().peekable();
            while let Some(c) = chars.next() {
                if inside_span {
                    current_span.push(c);
                    if current_span.ends_with("</span>") {
                        // Wrap the existing span content
                        span_buffer.push_str(&format!("<span onclick=\"window.translate(this)\">{}</span>", current_span));
                        modified_p_text.push_str(&span_buffer);
                        span_buffer.clear();
                        inside_span = false;
                        current_span.clear();
                    }
                } else if inside_tag {
                    tag_buffer.push(c);
                    if c == '>' {
                        inside_tag = false;
                        if inside_word {
                            modified_p_text.push_str("</span>");
                            inside_word = false;
                        }
                        modified_p_text.push_str(&tag_buffer);
                        tag_buffer.clear();
                    }
                } else if c == '<' {
                    if chars.peek() == Some(&'s') {
                        // Start of a <span> tag
                        current_span.push(c);
                        inside_span = true;
                    } else {
                        // Start of another tag
                        inside_tag = true;
                        tag_buffer.push(c);
                    }
                } else if c.is_whitespace() {
                    if inside_word {
                        modified_p_text.push_str("</span>");
                        inside_word = false;
                    }
                    modified_p_text.push(c);
                } else {
                    if !inside_word {
                        modified_p_text.push_str("<span onclick=\"window.translate(this)\">");
                        inside_word = true;
                    }
                    modified_p_text.push(c);
                }
            }
            if inside_word {
                modified_p_text.push_str("</span>");
            }

            let opening_tag = &html_string[index..opening_tag_end];
            let new_content = format!("{}{}</p>", opening_tag, modified_p_text);
            
            html_string.replace_range(index..closing_p_index + 4, &new_content);
            current_index = index + new_content.len();
        } else {
            panic!("Malformed html. Could not find closing p tag.");
        }
    }
    
    html_string
}

fn find_substring_from_index(string: &str, substring: &str, start_index: usize) -> Option<usize> {
    string[start_index..].find(substring).map(|index| index + start_index)
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_single_paragraph() {
//         let input = "<p>Hello world</p>";
//         let expected = r#"<p><span onclick="console.log(this.innerText)">Hello</span> <span onclick="console.log(this.innerText)">world</span></p>"#;
//         assert_eq!(wrap_words_in_paragraphs(input), expected);
//     }

//     #[test]
//     fn test_multiple_paragraphs() {
//         let input = "<p>First para</p><p>Second para</p>";
//         let expected = r#"<p><span onclick="console.log(this.innerText)">First</span> <span onclick="console.log(this.innerText)">para</span></p><p><span onclick="console.log(this.innerText)">Second</span> <span onclick="console.log(this.innerText)">para</span></p>"#;
//         assert_eq!(wrap_words_in_paragraphs(input), expected);
//     }

//     #[test]
//     fn test_non_paragraph_content() {
//         let input = "<div>This should not change</div>";
//         assert_eq!(wrap_words_in_paragraphs(input), input);
//     }

//     #[test]
//     fn test_mixed_content() {
//         let input = "<div>Unchanged</div><p>Changed content</p><span>Also unchanged</span>";
//         let expected = r#"<div>Unchanged</div><p><span onclick="console.log(this.innerText)">Changed</span> <span onclick="console.log(this.innerText)">content</span></p><span>Also unchanged</span>"#;
//         assert_eq!(wrap_words_in_paragraphs(input), expected);
//     }

//     #[test]
//     fn test_paragraph_with_whitespace() {
//         let input = "<p> Hello   world \n\t</p>";
//         let expected = r#"<p> <span onclick="console.log(this.innerText)">Hello</span>   <span onclick="console.log(this.innerText)">world</span> 
// 	</p>"#;
//         assert_eq!(wrap_words_in_paragraphs(input), expected);
//     }

//     #[test]
//     #[should_panic]
//     fn test_malformed_html() {
//         let input = "<p>Unclosed paragraph";
//         wrap_words_in_paragraphs(input);
//     }

//     #[test]
//     fn test_basic_find() {
//         let haystack = "Hello, world!";
//         assert_eq!(find_substring_from_index(haystack, "world", 0), Some(7));
//     }

//     #[test]
//     fn test_find_from_middle() {
//         let haystack = "Hello, world! Hello, Rust!";
//         assert_eq!(find_substring_from_index(haystack, "Hello", 7), Some(14));
//     }

//     #[test]
//     fn test_not_found() {
//         let haystack = "Hello, world!";
//         assert_eq!(find_substring_from_index(haystack, "Rust", 0), None);
//     }

//     #[test]
//     fn test_empty_needle() {
//         let haystack = "Hello, world!";
//         assert_eq!(find_substring_from_index(haystack, "", 0), Some(0));
//         assert_eq!(find_substring_from_index(haystack, "", 5), Some(5));
//     }

//     #[test]
//     fn test_empty_haystack() {
//         let haystack = "";
//         assert_eq!(find_substring_from_index(haystack, "test", 0), None);
//     }

//     #[test]
//     #[should_panic]
//     fn test_start_index_out_of_bounds() {
//         let haystack = "Hello, world!";
//         find_substring_from_index(haystack, "world", 20);
//     }

//     #[test]
//     fn test_start_index_at_boundary() {
//         let haystack = "Hello, world!";
//         assert_eq!(find_substring_from_index(haystack, "world", 7), Some(7));
//     }

//     #[test]
//     fn test_multiple_occurrences() {
//         let haystack = "test test test";
//         assert_eq!(find_substring_from_index(haystack, "test", 0), Some(0));
//         assert_eq!(find_substring_from_index(haystack, "test", 1), Some(5));
//         assert_eq!(find_substring_from_index(haystack, "test", 6), Some(10));
//     }

//     #[test]
//     fn test_unicode() {
//         let haystack = "Hello, 世界!";
//         assert_eq!(find_substring_from_index(haystack, "世界", 0), Some(7));
//     }

//     #[test]
//     fn test_wrap_words_with_single_attribute() {
//         let input = r#"<p class="test">This is a paragraph.</p>"#;
//         let expected = r#"<p class="test"><span onclick="console.log(this.innerText)">This</span> <span onclick="console.log(this.innerText)">is</span> <span onclick="console.log(this.innerText)">a</span> <span onclick="console.log(this.innerText)">paragraph.</span></p>"#;
//         assert_eq!(wrap_words_in_paragraphs(input), expected);
//     }

//     #[test]
//     fn test_wrap_words_with_multiple_attributes() {
//         let input = r#"<p id="para1" class="important" style="color: red;">Multiple attributes here.</p>"#;
//         let expected = r#"<p id="para1" class="important" style="color: red;"><span onclick="console.log(this.innerText)">Multiple</span> <span onclick="console.log(this.innerText)">attributes</span> <span onclick="console.log(this.innerText)">here.</span></p>"#;
//         assert_eq!(wrap_words_in_paragraphs(input), expected);
//     }

//     #[test]
//     fn test_wrap_words_with_attribute_and_multiple_paragraphs() {
//         let input = r#"<p class="first">First paragraph.</p><p>Second paragraph.</p><p id="last">Last paragraph.</p>"#;
//         let expected = r#"<p class="first"><span onclick="console.log(this.innerText)">First</span> <span onclick="console.log(this.innerText)">paragraph.</span></p><p><span onclick="console.log(this.innerText)">Second</span> <span onclick="console.log(this.innerText)">paragraph.</span></p><p id="last"><span onclick="console.log(this.innerText)">Last</span> <span onclick="console.log(this.innerText)">paragraph.</span></p>"#;
//         assert_eq!(wrap_words_in_paragraphs(input), expected);
//     }

//     #[test]
//     fn test_wrap_paragraph_with_existing_span() {
//         let input = "<p>Hello <span>world</span></p>";
//         let expected = "<p><span onclick=\"console.log(this.innerText)\">Hello</span> <span onclick=\"console.log(this.innerText)\"><span>world</span></span></p>";
//         assert_eq!(wrap_words_in_paragraphs(input), expected);
//     }

//     #[test]
//     fn test_wrap_multiple_spans() {
//         let input = "<p>Hello <span>beautiful</span> world</p>";
//         let expected = "<p><span onclick=\"console.log(this.innerText)\">Hello</span> <span onclick=\"console.log(this.innerText)\"><span>beautiful</span></span> <span onclick=\"console.log(this.innerText)\">world</span></p>";
//         assert_eq!(wrap_words_in_paragraphs(input), expected);
//     }

//     // #[test]
//     // fn test_wrap_words_with_attribute_containing_angle_brackets() {
//     //     let input = r#"<p data-content="<strong>Bold</strong>">Tricky attribute content.</p>"#;
//     //     let expected = r#"<p data-content="<strong>Bold</strong>"><span onclick="console.log(this.innerText)">Tricky</span> <span onclick="console.log(this.innerText)">attribute</span> <span onclick="console.log(this.innerText)">content.</span></p>"#;
//     //     assert_eq!(wrap_words_in_paragraphs(input), expected);
//     // }

// }
