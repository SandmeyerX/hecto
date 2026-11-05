---
title: "hecto：附录"
source: "https://philippflenker.com/hecto-appendices/"
author:
  - "[[Philipp Flenker]]"
published: 2024-03-30
---
## 带注释的提交如何工作

本教程中的大多数步骤都以带注释的提交链接的形式呈现。每个提交向你展示从前一步骤的代码到当前步骤所需做的更改，并附有解释每个有趣代码行的注释。

[这里是一个示例提交](https://github.com/pflenker/hecto-tutorial/commit/23f1c0f6c54d794a8794fa40432c8e52103cc188)。

每个 diff 对每个被修改的文件包含一个块。每个块以一个头部开始，包含你需要编辑的文件名（"src/main.rs"）。头部之后显示文件的内容。新增内容以绿色高亮，删除内容以红色高亮。旧行号打印在左侧，新行号打印在右侧。注释显示在每个有趣的代码行下方，标签 `(Author)` 告诉你提交的人和注释的人是同一个人（我）。

在右上角，你会找到两个重要的设置：

- split/unified 让你从上面描述的视图切换到左边显示旧文件、右边显示新文件的视图——以及切回。
- Whitespace/Ignore Whitespace 选择是否在 diff 中显示空白更改——例如，重新缩进代码时，改变的只是每行前面的空白而不是逻辑本身，所以简单地忽略这些更改可能是有意义的。  
	此外，在代码块的右上角，你会找到一个 `...` 按钮。点击它可以隐藏注释，专注于更改本身。  
	例如，[这个链接](https://github.com/pflenker/hecto-tutorial/commit/23f1c0f6c54d794a8794fa40432c8e52103cc188?diff=unified&w=0)带你到带注释的详细版本，帮助你推理每一行代码。[这个链接](https://github.com/pflenker/hecto-tutorial/commit/23f1c0f6c54d794a8794fa40432c8e52103cc188?diff=unified&w=1)带你到同一更改的更简洁版本，没有注释或空白更改。

## 如果你卡住了怎么办

最初的几个提交设计为让你跟随并复制它们。

本教程中的一些代码非常难以精确输入，特别是如果你不习惯 Rust。当你对一行进行更改时，很容易犯错，你认为你已经完成了对该行的更改，但漏掉了对同一行另一部分的一个小更改。慢慢来，并将 diff 中更改的部分与你的代码*逐字符*比较，以确保它们相同，这很重要。

如果你怀疑自己犯了错误，但不知道它在哪里或你可能在多早之前犯了错误，你应该让你的计算机对你版本的 `hecto` 和教程版本的 `hecto` 进行 diff，无论你处于哪一步。

你需要 `git` 来做到这一点。要安装 git，请遵循[这些官方说明](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)。安装 `git` 后，通过运行 `git clone https://github.com/pflenker/hecto-tutorial.git` 克隆 [hecto-tutorial](https://github.com/pflenker/hecto-tutorial) 仓库。使用 `cd hecto-tutorial` `cd` 进入仓库。每个提交链接的左上角都有一个提交 ID，你可以检出它。

要检出上面的提交，你会输入：

```bash
$ git checkout 2e090ab
```

然后该文件夹包含该步骤内容的所有文件。

你可以通过运行类似 `git diff --no-index -b ../path/to/your/main.rs main.rs` 的命令将你的文件与该文件夹的文件进行比较。这将向你展示你需要对 `main.rs` 进行的更改，以使其看起来像仓库中的那个。`-b` 选项忽略空白，所以如果你使用与教程不同的缩进风格，也不会有影响。

一段时间后，你会得到任务让你自己尝试解决。我建议使用我在任务之前的最后一个提交作为你自己实验的起点。这样，一旦你卡住了，你可以将你当前的状态与我的下一个提交进行比较。

要将你当前的代码与上面的提交进行比较，你会输入：

```bash
$ git diff 2e090ab
```

## 去哪里寻求帮助

获得帮助的最简单方法是直接在 GitHub 上评论你不理解的代码行。如果你的问题与特定代码行没有直接关系（或者你宁愿不使用 GitHub），并且上面的提示不足以帮助你，请随时联系我。你可以在[我的主页](https://philippflenker.com/)上找到几种联系我的方式。

## 改进和功能的想法

如果你想自己扩展 `hecto`，我建议试着实际使用 `hecto` 作为你的文本编辑器一段时间。你会很快痛苦地意识到你在文本编辑器中习惯拥有的各种功能，但 `hecto` 中缺少。这些就是你应该尝试添加的功能。而且当你做 `hecto` 的工作时，你应该使用 `hecto`。

如果你还在寻找想法，这里有一个小列表。如果你完成了其中一些，让我知道！

- 重构。我们的代码从无到有，我们沿途进行了重构，但我不想让这个教程变成重构盛宴，所以代码的结构没有它可能的那么好，这可能会使未来的 bug 修复和扩展更加困难。
- 测试。我们通过尝试和玩耍来测试我们的代码，但随着时间的推移这变得越来越困难，并且在添加新功能时很容易丢弃/破坏旧功能。是时候向 `hecto` 添加测试了。
- 偏执的 `clippy`。把 `clippy` 的行为调到 11 档，看看你是否能充分解决它提出的所有问题。
- 支持更多文件类型。
- 使高亮标记可配置。例如，在某些语言中，单个 `#` 而不是两个斜杠表示单行注释。
- 允许替代方案。例如，在许多语言中，字符不单独高亮，而是字符串可以用单引号或双引号分隔。
- 行号：在文件的每一行左侧显示行号。
- 自动缩进：开始新行时，将其缩进到与上一行相同的级别。
- 硬换行：当用户即将输入超过屏幕末尾时，在文本中插入换行符。尽量不要在会拆分单词的地方插入换行符。
- 软换行：当一行比屏幕宽度长时，在屏幕上使用多行来显示它，而不是水平滚动。
- 复制和粘贴：给用户一种选择文本的方式，然后在他们按下 Ctrl-C 时复制选中的文本，并让他们在按下 Ctrl-V 时粘贴复制的文本。
- 多个缓冲区：允许一次打开多个文件，并有某种在它们之间切换的方式。
- 非阻塞读取：`hecto` 目前在等待用户输入时完全阻塞，使得例如仅在特定时间内显示通知变得不可能。
- 鼠标支持：`crossterm` 提供对鼠标事件的支持。让用户通过点击终端来更改光标位置。

## 我为什么写 hecto

回到 2019 年（当我写这个教程的第一版时），我主要只熟悉 JavaScript。当然，我过去有一些 Java 经验，但多年来这些知识已经开始消退。

与此同时，Rust 是相当新的孩子，所以我在寻找学习它的方法。  
我的第一个想法是玩一个 web 服务器，但后来我想到这将接近我尝试闪亮新事物时总是做的事情。然后有一天，[Paige Ruten 出色的用 C 构建你自己的编辑器的教程](https://viewsourcecode.org/snaptoken/kilo/index.html)（再次）登上了 Hacker News 的头版，激发了这个教程的想法。`kilo` 足够复杂以构成挑战，而且感谢 Paige，它被充分理解，所以我可以专注于学习语言而不会迷失在实现文本编辑器的细节中。

## 名字

`hecto` 追求的目标比 `kilo` 更谦虚。毕竟，我只是拿了一个预先存在的想法并重写了它，所以为我的最终项目选择一个“更小”的名字似乎是明智的。[^1]

## 贡献

欢迎贡献！有三种类型：

- 对**文本**的贡献，例如额外的解释、更正或其他补充：请通过文本发送给我，并包括你希望如何被署名。请注意，我可能不会复制粘贴你的文本，而是将其融入我的文本中，以便教程保持一致的语调。我仍然会在更改的文本部分直接以脚注形式署名你的贡献。
- 对**步骤**的贡献，例如 bug 修复：鉴于仓库的性质，这些工作方式略有不同。请通过上面的命令检出你要在其上构建 bug 修复的最新提交。然后使用 `git branch my-new-branch` 基于此提交创建一个分支，然后使用 `git checkout my-new-branch` 切换到它。确保你的更改在一个单独的提交中完成，然后打开一个 PR 并评论你的提交。在你的 PR 中，请让我知道你想如何被署名。然后我将在适当的位置链接你的提交，并附上一些解释和署名。
- **扩展**：如果你完成了上面的某个功能添加，或以任何其他方式扩展了 `hecto`，那绝对太棒了！只需打开一个 PR，广泛地评论它（如果你愿意），并告诉我你想如何被署名。不要担心像上一步中我要求你的那样把所有东西保持在一个提交中。然后我将把你的贡献和署名添加到上面的列表中。  
	请注意，我目前不提供行为准则，并且向我发送任何贡献并不保证它会进入教程文本或代码。这是我的个人项目，不是我的全职工作。请友善。

## 致谢

[Salvatore Sanfilippo 又名 antirez](http://invece.org/) 是 [kilo](https://github.com/antirez/kilo) 的作者。他写了一篇[博客文章](http://antirez.com/news/108)关于它，在其中他解释了他如何重用他另外两个项目中的代码，在几个已经忙碌的周末里仅用几个小时就快速拼凑出 `kilo`。  
`kilo` 是 [Paige Ruten](https://viewsourcecode.org/) 构建她自己版本的 `kilo` 的起点。她是如何做到的，在[她的一篇文章](https://viewsourcecode.org/snaptoken/kilo/08.appendices.html#credits)中有更详细的描述，那篇文章与这篇惊人地相似。然后她将其变成了一个令人难以置信的教程，叫做[“构建你自己的文本编辑器”](https://viewsourcecode.org/snaptoken/kilo/index.html)，这基本上就是我改编成你现在看到的教程的内容。如果你想了解更多关于 Paige 的信息，请访问 [viewsourcecode.org](http://viewsourcecode.org/)。

我见过 Paige 的教程频繁出现在 [Hacker News](https://news.ycombinator.com/news) 上，并且总是想着实际实现它。相反，我将她的教程与我“以 Rust 的方式”学习 Rust 的努力结合起来。

## 许可证

- `kilo` 源代码在 [BSD 2-Clause](https://github.com/snaptoken/kilo-tutorial/blob/master/steps.diff.LICENSE) 许可证下发布。
- [原始教程](https://viewsourcecode.org/snaptoken/kilo/)在 [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) 下许可。
- `hecto` 和本教程在 [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/) 下许可。请署名我为 Philipp Flenker 并链接[我的主页。](https://philippflenker.com/)

### 署名与变更说明

由于本作品基于 CC BY 4.0 许可的先前作品，我需要正确署名原始作者并说明我对原始作品所做的更改，我很乐意在这里这样做：

- 原始的 C 程序 `kilo` 由 [antirez](http://invece.org/) 完成。
- 基于 `kilo` 构建的教程由 [Paige Ruten](https://viewsourcecode.org/) 完成。  
	这些博客文章牢固地基于[原始教程](https://viewsourcecode.org/snaptoken/kilo/index.html)，但代码不仅通过使用最接近的 Rust 对应物适配到 Rust，而且通过尝试“以 Rust 的方式”解决问题。原始的所有解释都已检查、修订、在适当的地方适配到 Rust、在我认为有价值的地方扩展，或在我认为需要时重写。
- 小型文本编辑器的想法和 `hecto` 的功能来自 antirez，因为 `hecto` 是对原始 `kilo` 的近乎忠实的重实现。
- 将构建编辑器变成教程的想法和教程的结构是 Paige 的工作。步骤的顺序也是如此，尽管我在我认为有意义的地方改变了一些东西。
- 每当你读到关于终端如何工作的有趣内容时，那大多是 Paige 的工作，由我添加和扩展了一些内容。
- 所有与 Rust 相关的内容以及关于编码的一般思考都来自我。  
	对于任何你无法用此指南清晰归属的内容，以下规则可能适用：如果有见地或机智，那是 Paige。如果冗余、不言自明、愚蠢或冗余，那是我。

## 致谢

感谢：

- [Paige Ruten](https://viewsourcecode.org/) 基于 `kilo` 的 [C 教程](https://viewsourcecode.org/snaptoken/kilo/index.html)
- [antirez](http://invece.org/) 的 `kilo`
- 以下人员为原始代码库提供了 bug 修复和其他 Pull Request。谢谢！
	- [Stephen S.H.](https://github.com/2222-42)
		- [Avinash Sajjanshetty](https://github.com/avinassh)
		- [Jaromir Obr](https://github.com/mirao)
		- [Eddie Nuno](https://github.com/archaengel)
		- [Junhyeok Jang](https://github.com/kennethjang34)
- 每个花时间给我发邮件表示感谢或鼓励的人。你们太棒了！

*这篇文章是教程 [hecto：用 Rust 构建你自己的文本编辑器](hecto.md) 的一部分。在[这里](hecto.md)找到更多信息，特别是关于许可和致谢的信息。*

---

[^1]: 解释：`kilo` 来自希腊语，意思是 1000，因为这个程序最初的目标是在少于 1000 行内构建一个文本编辑器。`hecto` 意思是 100。