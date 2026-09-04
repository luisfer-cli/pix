document.querySelectorAll('[data-copy]').forEach((button) => {
  button.addEventListener('click', async () => {
    const target = document.getElementById(button.dataset.copy);
    if (!target) return;
    await navigator.clipboard.writeText(target.textContent.trim());
    const previous = button.textContent;
    button.textContent = 'Copiado ✓';
    setTimeout(() => { button.textContent = previous; }, 1400);
  });
});
