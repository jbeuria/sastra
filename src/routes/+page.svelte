<script>
import TreeViewer from '$lib/components/toc/treeview.svelte';
import bookIcon from '$lib/components/icons/book_icon.png'
let tree = [
      {
        desc: "Javascript",
        child: [
          {
            desc: "Learn React without learning basics 😅",
            child: [
                  {
                    desc: "Learn very Advanced Java 1",
                    child: [
                  {
                    desc: "Learn very Advanced Java 1",
                    child: [
                  {
                    desc: "Learn very Advanced Java 1",
                    child: [
                  {
                    desc: "Learn very Advanced Java 1",
                    key: "javaVAD1"
                  },
                  {
                    desc: "Learn very Advanced Java 2",
                    child: [
                  {
                    desc: "Learn very Advanced Java 1",
                    child: [
                  {
                    desc: "Learn very Advanced Java 1",
                    child: [
                  {
                    desc: "Learn very Advanced Java 1",
                    child: [
                  {
                    desc: "Learn very Advanced Java 1",
                    key: "javaVAD1"
                  },
                  {
                    desc: "Learn very Advanced Java 2",
                    key: "javaVAD2"
                  }
                ]
                  },
                  {
                    desc: "Learn very Advanced Java 2",
                    key: "javaVAD2"
                  }
                ]
                  },
                  {
                    desc: "Learn very Advanced Java 2",
                    key: "javaVAD2"
                  }
                ]
                  },
                  {
                    desc: "Learn very Advanced Java 2",
                    key: "javaVAD2"
                  }
                ]
                  }
                ]
                  },
                  {
                    desc: "Learn very Advanced Java 2",
                    key: "javaVAD2"
                  }
                ]
                  },
                  {
                    desc: "Learn very Advanced Java 2",
                    key: "javaVAD2"
                  }
                ]
                  },
                  {
                    desc: "Learn very Advanced Java 2",
                    key: "javaVAD2"
                  }
                ]
          },
          {
            desc: "Learn Angular without learning basics 😅",
            key: "jsA"
          },
          {
            desc: "Learn JS from scratch 🤓",
            key: "jsB"
          }
        ]
      },
      {
        desc: "Python",
        child: [
          {
            desc: "Learn Machine Learning without learning basics 😅",
            child: [
              {
                desc: "Lecture 1",
                child: [
                  {
                    desc: "Learn very Advanced Java 1",
                    key: "javaVAD1"
                  },
                  {
                    desc: "Learn very Advanced Java 2",
                    key: "javaVAD2"
                  }
                ]
              }
            ]
          },
          {
            desc: "Learn Python Basics 🐍🐍",
            key: "pyB"
          }
        ]
      },
      {
        desc: "Learn Java",
        child: [
          {
            desc: "Learn Advanced Java",
            child: [
              {
                desc: "Learn very Advanced Java",
                child: [
                  {
                    desc: "Learn very Advanced Java 1",
                    key: "javaVAD1"
                  },
                  {
                    desc: "Learn very Advanced Java 2",
                    key: "javaVAD2"
                  }
                ]
              },
              {
                desc: "Learn very very Advanced Java",
                key: "javaVVAD"
              }
            ]
          },
          {
            desc: "Learn Java basics",
            key: "javaB"
          }
        ]
      }
    ];
</script>


<TreeViewer tree={tree} secondaryIcon={bookIcon} />