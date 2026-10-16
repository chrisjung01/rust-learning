// Shared quiz widget.
// Markup: <div class="quiz"><div class="q">…</div>
//           <button>…</button> <button data-correct>…</button> …
//           <div class="feedback">explanation</div></div>
// Optional: <p class="score" id="score"></p> anywhere on the page.
document.addEventListener("DOMContentLoaded", () => {
  const quizzes = [...document.querySelectorAll(".quiz")];
  let answered = 0, correct = 0;
  const score = document.getElementById("score");
  const update = () => {
    if (score) score.textContent = `${correct} / ${answered} correct (of ${quizzes.length})`;
  };
  quizzes.forEach((quiz) => {
    const buttons = [...quiz.querySelectorAll("button")];
    buttons.forEach((b) =>
      b.addEventListener("click", () => {
        buttons.forEach((x) => {
          x.disabled = true;
          if (x.hasAttribute("data-correct")) x.classList.add("right");
        });
        if (!b.hasAttribute("data-correct")) b.classList.add("wrong");
        else correct++;
        answered++;
        quiz.classList.add("done");
        update();
      })
    );
  });
  update();
});
