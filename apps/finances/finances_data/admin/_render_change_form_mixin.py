class RenderChangeFormMixin:
    """A mixin that shows the 'help_text' message at the top of the change form in the Django admin

    To use it, just inherit from this mixin and add a 'change_form_help_text' class attribute

    ```py
    class SomeModelAdmin(RenderChangeFormMixin, admin.ModelAdmin):
        change_form_help_text = "<strong>Note.-</strong>. My notes"

    admin.site.register(SomeModel, SomeModelAdmin)
    ```
    """

    def render_change_form(self, request, context, *args, **kwargs):
        self.change_form_template = "admin/finances_data/change_form.html"
        extra = {"help_text": self.change_form_help_text}

        context.update(extra)
        return super().render_change_form(request, context, *args, **kwargs)
